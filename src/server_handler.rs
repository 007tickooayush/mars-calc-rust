use crate::method::Method;
use crate::model_request::Request;
use crate::model_response::Response;
use crate::model_status_code::StatusCode;
use crate::server::Handler;
use crate::server_handler_functions::{demo_request, health_check_server, read_file_securely, test_req_body_post, wildcard_response};

pub struct ServerHandler {
    public_path: String
}

impl ServerHandler {
    pub fn new(public_path: String) -> Self {
        Self {
            public_path
        }
    }

    /// A function to provide a file from the server
    pub fn read_file(&self, file_path: &str) -> Option<String> {
        let complete_file_path = format!("{}/{}", self.public_path, file_path);


        // SUGGEST: XX This is not a secure method as it opens up the directory traversal security issue e.g. GET /../Cargo.toml XX
        // SUGGEST: Using .ok() method because it returns value if there is a values, else it returns none
        // std::fs::read_to_string(complete_file_path).ok()
        // SUGGEST: XX This is not a secure method as it opens up the directory traversal security issue e.g. GET /../Cargo.toml XX

        match std::fs::canonicalize(complete_file_path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    std::fs::read_to_string(path).ok()
                } else {
                    eprintln!("Directory traversal attack attempted: {}", file_path);
                    None
                }
            },
            Err(e) => {
                None
            }
        }
    }
}

impl Handler for ServerHandler {

    /// Handling all the Request Methods and providing static response
    /// NOTE: For other HTTP Protocol Request Methods, the methods available in the Method enum can be added in the body to handle the request and response
    fn handle_request(&mut self, request: &Request) -> Response {
        // dbg!(request);
        // Response::new(StatusCode::Ok, Some(String::from("Hello, World!")))

        match request.method() {
            Method::GET  => match request.path() {
                "/" => health_check_server(&request),
                "/hello" => demo_request(&request),
                // "/file" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                path => read_file_securely(&request, self, path),
                _ => wildcard_response()
            },
            // SUGGEST: Add further Methods Method::POST, Method::PUT, Method::DELETE
            Method::POST => match request.path() {
                "/test-body" => test_req_body_post(&request, self),
                _ => wildcard_response()
            }
            _ => wildcard_response()
        }
    }
}

