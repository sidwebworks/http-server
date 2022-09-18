use crate::http::response::Response;
use crate::http::{ParseError, Request, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpStream;
use std::{net::TcpListener, process};

pub trait Handler {
    fn handle_request(&self, request: &Request) -> Response;
    fn handle_bad_request(&self, err: &ParseError) -> Response {
        println!("Failed to parse request: {}", err);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    address: String,
    handler: Option<Box<dyn Handler>>,
}

impl Server {
    pub fn new(host: &str, port: i32) -> Self {
        Server {
            address: format!("{}:{}", host, port),
            handler: None,
        }
    }

    pub fn listen(self) {
        let listener = TcpListener::bind(&self.address);

        match listener {
            Ok(handle) => {
                println!("Server listening at {}", self.address);
                loop {
                    match handle.accept() {
                        Ok((mut stream, _)) => {
                            println!("Connection established\n");

                            let mut buffer = [0; 1024];

                            match stream.read(&mut buffer) {
                                Ok(_) => {
                                    println!("Recieved a request: \n");

                                    let handler = self.handler.as_deref().expect("Request handler is required");

                                    let response = match Request::try_from(&buffer[..]) {
                                        Ok(request) => handler.handle_request(&request),
                                        Err(err) => handler.handle_bad_request(&err),
                                    };

                                    if let Err(e) = response.send(&mut stream) {
                                        println!("Failed to send response: {}", e);
                                    }
                                }
                                Err(err) => {
                                    println!("Failed to read from connection: {}", err)
                                }
                            };
                        }
                        Err(err) => {
                            println!("Failed to establish a connection: {}", err.kind());
                        }
                    }
                }
            }
            Err(err) => {
                println!("[HTTP-SERVER]: {}", err.kind());
                process::exit(1);
            }
        }
    }

    pub fn set_handler(&mut self, handler: Option<Box<dyn Handler>>) {
        self.handler = handler;
    }
}
