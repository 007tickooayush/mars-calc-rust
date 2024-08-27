use crate::model_request::Request;
use crate::model_response::Response;
use crate::model_status_code::StatusCode;
use crate::server_handler::ServerHandler;

pub fn health_check_server(request: &Request) -> Response {
    println!("Request Object: {:?}", request);
    Response::new(StatusCode::Ok, Some(String::from("The server is running!")))
}

pub fn demo_request(_request: &Request) -> Response {
    Response::new(StatusCode::Ok, Some(String::from("Hi how are you!")))
}

pub fn read_file_securely(_request: &Request, handler: &mut ServerHandler, path: &str) -> Response {
    match handler.read_file(path) {
        Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
        None => Response::new(StatusCode::NotFound, None)
    }
}

pub fn test_req_body_post(_request: &Request, _handler: &mut ServerHandler) -> Response {
    println!("{:?}", _request);
    Response::new(StatusCode::Ok, None)
    // Response::new(StatusCode::Ok, Some(format!("The request body is: {}", request.body())))
}

pub fn wildcard_response() -> Response {
    Response::new(StatusCode::NotFound, None)
}