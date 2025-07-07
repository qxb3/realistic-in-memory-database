mod db;

use std::sync::Arc;

use tiny_http::{Header, Method, Request, Response, Server};
use tokio::sync::Mutex;

use crate::db::{Data, DataValue, Db, Id};

#[tokio::main]
async fn main() {
    let server = Server::http("0.0.0.0:4321").unwrap();
    println!("Server listening on http://127.0.0.1:4321");

    let db = Arc::new(Mutex::new(Db::new()));

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

async fn handle_request(request: Request, db: Arc<Mutex<Db>>) {
    // Only handle request from /db.
    if !request.url().starts_with("/db") {
        request.respond(Response::from_string("{ \"status\": \"error\", \"message\": \"go to /db pls\" }").with_status_code(400)).unwrap();
        return;
    }

    match request.method() {
        Method::Post => handle_create(request, db).await,
        Method::Get => handle_read(request, db).await,
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
