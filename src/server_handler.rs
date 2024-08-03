use crate::model_request::Request;
use crate::model_response::Response;
use crate::model_status_code::StatusCode;
use crate::server::Handler;

pub struct ServerHandler;

impl Handler for ServerHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        dbg!(request);
        Response::new(StatusCode::Ok, Some(String::from("Hello, World!")))
    }
}