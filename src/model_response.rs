pub struct Response {
    status_code : StatusCode,
    body: Option<String>
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }
}

pub enum StatusCode {
    OK = 200,
    BAD_REQUEST = 400,
    NOT_FOUND = 404,
}