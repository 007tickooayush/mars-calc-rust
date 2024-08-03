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
                _ => Response::new(StatusCode::NotFound, None)
            },
            // SUGGEST: Add further Methods Method::POST, Method::PUT, Method::DELETE
            _ => Response::new(StatusCode::NotFound, None)
        }
    }
}

