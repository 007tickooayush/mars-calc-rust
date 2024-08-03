use crate::method::Method;
use crate::model_request::Request;
use crate::model_response::Response;
use crate::model_status_code::StatusCode;
use crate::server::Handler;

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
    fn read_file(&self, file_path: &str) -> Option<String> {
        let complete_file_path = format!("{}/{}", self.public_path, file_path);
        // SUGGEST: Using .ok() method because it returns value if there is a values, else it returns none
        std::fs::read_to_string(complete_file_path).ok()
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
                "/" => Response::new(StatusCode::Ok, Some(String::from("The server is running!"))),
                "/hello" => Response::new(StatusCode::Ok, Some(String::from("Hi how are you!"))),
                "/file" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                _ => Response::new(StatusCode::NotFound, None)
            },
            // SUGGEST: Add further Methods Method::POST, Method::PUT, Method::DELETE
            _ => Response::new(StatusCode::NotFound, None)
        }
    }
}

