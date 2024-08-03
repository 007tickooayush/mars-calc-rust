use std::fmt::{Display, Formatter};
use std::net::TcpStream;
use std::io::Write;
use crate::model_status_code::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    /// utilizes the std::io::Result to write into the stream
    /// and without creating a new instance of Response to be written first to the Formatter,
    /// XX-directly send the response to the client using the TcpStream=XX
    /// UPDATE: NOT USING TcpStream strict type, instead using static Dispatch of `Write` trait, so that a String, File, etc.,
    /// all the response types that are implementors of Write trait can utilize this function
    pub fn send(&self, stream: &mut impl Write) -> std::io::Result<()> {
        let body = match &self.body {
            Some(b) => b,
            None => ""
        };

        write!(stream, "HTTP/1.1 {} {}\r\n\r\n{}", self.status_code, self.status_code.reason(), body)
    }
}

// impl Display for Response {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let body = match &self.body {
//             Some(b) => b,
//             None => ""
//         };
//
//         write!(f, "HTTP/1.1 {} {}\r\n\r\n{}", self.status_code, self.status_code.reason(), body)
//     }
// }