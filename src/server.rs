use crate::http::Request;
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpStream;
use std::{net::TcpListener, process};

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(addr: &str) -> Self {
        Server {
            address: addr.to_string(),
        }
    }

    pub fn listen(self) {
        let listener = TcpListener::bind(&self.address);

        match listener {
            Ok(handle) => {
                println!("Server listening at {}", self.address);
                loop {
                    match handle.accept() {
                        Ok((stream, _)) => {
                            handle_connection(stream);
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
}

fn handle_connection(mut stream: TcpStream) {
    println!("Connection established\n");

    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(_) => {
            println!("Recieved a request: \n");

            match Request::try_from(&buffer[..]) {
                Ok(request) => {
                    dbg!(request);
                }
                Err(err) => println!("Failed to parse request: {}", err),
            }
        }
        Err(err) => {
            println!("Failed to read from connection: {}", err)
        }
    }
}
