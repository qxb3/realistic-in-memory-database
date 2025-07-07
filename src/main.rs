mod db;

use std::{sync::Arc, thread, time::Duration};

use tiny_http::{Header, Method, Request, Response, Server};
use tokio::sync::Mutex;

use crate::db::{Data, DataValue, Db, Id};

#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(Db::new()));

    start_forgeting(Arc::clone(&db));
    start_server(Arc::clone(&db));
}

fn start_server(db: Arc<Mutex<Db>>) {
    let server = Server::http("0.0.0.0:4321").unwrap();
    println!("Server listening on http://127.0.0.1:4321");

    loop {
        let request = match server.recv() {
            Ok(request) => request,
            Err(e) => {
                eprintln!("Failed to receive event: {e}");
                continue;
            }
        };

        let db = Arc::clone(&db);
        tokio::spawn(async move {
            handle_request(request, db).await;
        });
    }
}

fn start_forgeting(db: Arc<Mutex<Db>>) {
    tokio::spawn(async move {
        loop {
            thread::sleep(Duration::from_secs(2));
            let mut db = db.lock().await;
            db.forget_random();
        }
    });
}

async fn handle_request(request: Request, db: Arc<Mutex<Db>>) {
    // Only handle request from /db.
    if !request.url().starts_with("/db") {
        request.respond(Response::from_string("{ \"status\": \"error\", \"message\": \"go to /db pls\" }").with_status_code(400)).unwrap();
        return;
    }

    match request.method() {
        Method::Post => handle_create(request, db).await,
        Method::Get => handle_read(request, db).await,
        Method::Trace => handle_list(request, db).await,
        Method::Patch => handle_update(request, db).await,
        Method::Delete => handle_delete(request, db).await,
        _ => {}
    }
}

async fn handle_create(request: Request, db: Arc<Mutex<Db>>) {
    let header_data = match request.headers().iter().find(|h| h.field.as_str() == "Data") {
        Some(header_id) => header_id.value.to_string(),
        None => {
            request.respond(Response::from_string("{ \"status\": \"error\", \"message\": \"Header: Data is required\" }").with_status_code(400)).unwrap();
            return;
        }
    };

    let data_value = DataValue::from_string(header_data);
    let data = Data::new(data_value);

    let mut db = db.lock().await;
    db.create(data);

    request.respond(
        Response::from_string("{ \"status\": \"ok\" }")
            .with_header(Header::from_bytes("Content-Type", "application/json").unwrap())
            .with_status_code(200)
    ).unwrap();
}

async fn handle_read(request: Request, db: Arc<Mutex<Db>>) {
    let header_id = match request.headers().iter().find(|h| h.field.as_str() == "Data-Id") {
        Some(header_id) => header_id.value.to_string(),
        None => {
            request.respond(Response::from_string("{ \"status\": \"error\", \"message\": \"Header: Data-Id is required\" }").with_status_code(400)).unwrap();
            return;
        }
    };

    let id: Id = if let Ok(value) = header_id.parse::<u64>() {
        value
    } else {
        request.respond(Response::from_string("{ \"status\": \"error\", \"message\": \"Invalid Id\" }").with_status_code(400)).unwrap();
        return;
    };

    let db = db.lock().await;
    let data = match db.read(id) {
        Some(data) => data,
        None => {
            request.respond(Response::from_string("{ \"status\": \"error\", \"message\": \"Cannot read data with that id\" }").with_status_code(400)).unwrap();
            return;
        }
    };

    request.respond(
        Response::from_string(format!("{{ \"status\": \"ok\", \"data\": {} }}", data.value))
            .with_header(Header::from_bytes("Content-Type", "application/json").unwrap())
            .with_status_code(200)
    ).unwrap();
}

async fn handle_list(request: Request, db: Arc<Mutex<Db>>) {
    let db = db.lock().await;
    let data_list = db.list();

    let mut list = String::new();
    list.push('[');

    for (i, (id, data)) in data_list.iter().enumerate() {
        list.push_str(
            format!(
                "{{ \"id\": {id}, \"data\": {} }} {}",
                data.value,
                if i == data_list.len() - 1 {
                    ""
                } else {
                    ","
                }
            ).as_str()
        );
    }

    list.push(']');

    request.respond(Response::from_string(list)).unwrap();
}

async fn handle_update(request: Request, db: Arc<Mutex<Db>>) {
    let header_id = match request.headers().iter().find(|h| h.field.as_str() == "Data-Id") {
        Some(header_id) => header_id.value.to_string(),
        None => {
            request.respond(Response::from_string("{ \"status\": \"error\", \"message\": \"Header: Data-Id is required\" }").with_status_code(400)).unwrap();
            return;
        }
    };

    let id: Id = if let Ok(value) = header_id.parse::<u64>() {
        value
    } else {
        request.respond(Response::from_string("{ \"status\": \"error\", \"message\": \"Invalid Id\" }").with_status_code(400)).unwrap();
        return;
    };

    let header_new_data = match request.headers().iter().find(|h| h.field.as_str() == "New-Data") {
        Some(header_new_data) => header_new_data.value.to_string(),
        None => {
            request.respond(Response::from_string("{ \"status\": \"error\", \"message\": \"Header: New-Data is required\" }").with_status_code(400)).unwrap();
            return;
        }
    };

    let new_data_value = DataValue::from_string(header_new_data);
    let new_data = Data::new(new_data_value);

    let mut db = db.lock().await;
    match db.update(id, new_data) {
        Ok(_) => {
            request.respond(
                Response::from_string("{ \"status\": \"ok\" }")
                    .with_header(Header::from_bytes("Content-Type", "application/json").unwrap())
                    .with_status_code(200)
            ).unwrap();
        },
        Err(e) => {
            request.respond(Response::from_string(e).with_status_code(400)).unwrap();
        }
    };
}

async fn handle_delete(request: Request, db: Arc<Mutex<Db>>) {
    let header_id = match request.headers().iter().find(|h| h.field.as_str() == "Data-Id") {
        Some(header_id) => header_id.value.to_string(),
        None => {
            request.respond(Response::from_string("{ \"status\": \"error\", \"message\": \"Header: Data-Id is required\" }").with_status_code(400)).unwrap();
            return;
        }
    };

    let id: Id = if let Ok(value) = header_id.parse::<u64>() {
        value
    } else {
        request.respond(Response::from_string("{ \"status\": \"error\", \"message\": \"Invalid Id\" }").with_status_code(400)).unwrap();
        return;
    };

    let mut db = db.lock().await;
    match db.delete(id) {
        Ok(_) => {
            request.respond(
                Response::from_string("{ \"status\": \"ok\" }")
                    .with_header(Header::from_bytes("Content-Type", "application/json").unwrap())
                    .with_status_code(200)
            ).unwrap();
        },
        Err(e) => {
            request.respond(Response::from_string(e).with_status_code(400)).unwrap();
        }
    }
}
