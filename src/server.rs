use std::io::{Read, Write};
use std::net::TcpListener;
use crate::model_request::Request;
use crate::model_response::Response;
use crate::model_status_code::StatusCode;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn start(&self) {
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
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    let response = Response::new(StatusCode::Ok, None);
                                    write!(stream, "{}", response).unwrap();
                                },
                                Err(e) => {
                                    eprintln!("Failed to parse a request: {}", e);
                                }
                            }
                        },
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

