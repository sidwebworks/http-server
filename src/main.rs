use server::{Handler, Server};

use crate::http::{Response, StatusCode};

mod http;
mod server;

struct RequestHandler;

impl Handler for RequestHandler {
    fn handle_request(&self, request: &http::Request) -> Response {
        dbg!(request);
        Response::new(
            StatusCode::Ok,
            Some("<h1>Hello world its working </h1>".to_string()),
        )
    }
}

fn main() {
    let mut server = Server::new("127.0.0.01", 4000);
    let handler = Box::new(RequestHandler);

    server.set_handler(Some(handler));

    server.listen()
}
