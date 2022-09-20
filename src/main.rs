use std::{env, fs};

use server::{Handler, Server};

use crate::http::{Method, Response, StatusCode};

mod http;
mod server;

struct RequestHandler {
    public_path: String,
}

impl RequestHandler {
    pub fn new(public_path: String) -> Self {
        RequestHandler { public_path }
    }
    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory traversal attempt");
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for RequestHandler {
    fn handle_request(&self, request: &http::Request) -> Response {
        dbg!(request);

        match request.method() {
            Method::GET => match request.path() {
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}

fn main() {
    let mut server = Server::new("127.0.0.01", 4000);
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR")).to_string();
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);

    let handler = Box::new(RequestHandler::new(public_path));

    server.set_handler(Some(handler));

    server.listen()
}
