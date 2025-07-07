mod db;

use rand::Rng;
use tiny_http::{Header, Method, Response, Server};

use crate::db::{Data, Db, Id};

fn main() {
    let server = Server::http("0.0.0.0:4321").unwrap();
    println!("Server listening on http://127.0.0.1:4321");

    let mut db = Db::new();
    let mut rand = rand::rng();

    for request in server.incoming_requests() {
        if request.url() != "db" {
            continue;
        }

        let headers = request.headers();

        // Get.
        if *request.method() == Method::Post {
            let id: Id = rand.random();

            let header_data = match headers.iter().find(|h| h.field.as_str() == "Data") {
                Some(header_id) => header_id.value.to_string(),
                None => {
                    request.respond(Response::from_string("{ \"status\": \"error\", \"message\": \"Header: Data is required\" }").with_status_code(400)).unwrap();
                    continue;
                }
            };

            let data = Data::from_string(header_data);

            db.create(id, data);

            request.respond(
                Response::from_string("{ \"status\": \"ok\" }")
                    .with_header(Header::from_bytes("Content-Type", "application/json").unwrap())
                    .with_status_code(200)
            ).unwrap();

            continue;
        }

        // Read.
        if *request.method() == Method::Get {
            let header_id = match headers.iter().find(|h| h.field.as_str() == "Data-Id") {
                Some(header_id) => header_id.value.to_string(),
                None => {
                    request.respond(Response::from_string("{ \"status\": \"error\", \"message\": \"Header: Data-Id is required\" }").with_status_code(400)).unwrap();
                    continue;
                }
            };

            let id: Id = if let Ok(value) = header_id.parse::<u64>() {
                value
            } else {
                request.respond(Response::from_string("{ \"status\": \"error\", \"message\": \"Invalid Id\" }").with_status_code(400)).unwrap();
                continue;
            };

            let data = match db.read(id) {
                Some(data) => data,
                None => {
                    request.respond(Response::from_string("{ \"status\": \"error\", \"message\": \"Cannot read data with that id\" }").with_status_code(400)).unwrap();
                    continue;
                }
            };

            request.respond(
                Response::from_string(format!("{{ \"status\": \"ok\", \"data\": {} }}", data))
                    .with_header(Header::from_bytes("Content-Type", "application/json").unwrap())
                    .with_status_code(200)
            ).unwrap();

            continue;
        }

        // Update.
        if && *request.method() == Method::Patch {
            let header_id = match headers.iter().find(|h| h.field.as_str() == "Data-Id") {
                Some(header_id) => header_id.value.to_string(),
                None => {
                    request.respond(Response::from_string("{ \"status\": \"error\", \"message\": \"Header: Data-Id is required\" }").with_status_code(400)).unwrap();
                    continue;
                }
            };

            let id: Id = if let Ok(value) = header_id.parse::<u64>() {
                value
            } else {
                request.respond(Response::from_string("{ \"status\": \"error\", \"message\": \"Invalid Id\" }").with_status_code(400)).unwrap();
                continue;
            };

            let header_new_data = match headers.iter().find(|h| h.field.as_str() == "New-Data") {
                Some(header_new_data) => header_new_data.value.to_string(),
                None => {
                    request.respond(Response::from_string("{ \"status\": \"error\", \"message\": \"Header: New-Data is required\" }").with_status_code(400)).unwrap();
                    continue;
                }
            };

            let new_data = Data::from_string(header_new_data);

            match db.update(id, new_data) {
                Ok(_) => {
                    request.respond(
                        Response::from_string("{ \"status\": \"ok\" }")
                            .with_header(Header::from_bytes("Content-Type", "application/json").unwrap())
                            .with_status_code(200)
                    ).unwrap();

                    continue;
                },
                Err(e) => {
                    request.respond(Response::from_string(e).with_status_code(400)).unwrap();
                    continue;
                }
            };
        }

        // Delete.
        if *request.method() == Method::Delete {
            let header_id = match headers.iter().find(|h| h.field.as_str() == "Data-Id") {
                Some(header_id) => header_id.value.to_string(),
                None => {
                    request.respond(Response::from_string("{ \"status\": \"error\", \"message\": \"Header: Data-Id is required\" }").with_status_code(400)).unwrap();
                    continue;
                }
            };

            let id: Id = if let Ok(value) = header_id.parse::<u64>() {
                value
            } else {
                request.respond(Response::from_string("{ \"status\": \"error\", \"message\": \"Invalid Id\" }").with_status_code(400)).unwrap();
                continue;
            };

            match db.delete(id) {
                Ok(_) => {
                    request.respond(
                        Response::from_string("{ \"status\": \"ok\" }")
                            .with_header(Header::from_bytes("Content-Type", "application/json").unwrap())
                            .with_status_code(200)
                    ).unwrap();

                    continue;
                },
                Err(e) => {
                    request.respond(Response::from_string(e).with_status_code(400)).unwrap();
                    continue;
                }
            }
        }
    }
}
