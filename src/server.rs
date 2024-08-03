use std::io::{Read, Write};
use std::net::TcpListener;
use crate::error::ParseError;
use crate::model_request::Request;
use crate::model_response::Response;
use crate::model_status_code::StatusCode;

/// Handler trait for server
pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    /// since the implementation of bad request might not be required to be implemented for every case
    /// hence the provide a default implementation
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        eprintln!("Failed to parse a request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn start(&self, mut handler: impl Handler) {
        println!("Server started at: {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        // using the Rust's infinite "loop" for listening to the incoming connection requests
        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    println!("\nNew client: {}", addr);
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            // SYNTAX: another WAY
                            // SYNTAX: $buffer as &[u8]
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    handler.handle_request(&request)
                                }
                                Err(e) => {
                                    handler.handle_bad_request(&e)
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                eprintln!("Failed to send a response: {}", e);
                            }
                        }
                        Err(_) => {
                            eprintln!("Failed to read from connection");
                        }
                    };
                }
                Err(e) => {
                    eprintln!("Failed to establish a connection: {}", e);
                }
            }
        }
    }
}

