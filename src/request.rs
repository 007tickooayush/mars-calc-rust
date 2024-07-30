use crate::method::Method;
use std::convert;
pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl Request {

    /// Parse the bytes array from the incoming request and return a Request instance
    /// and the error message type is Custom, not the traditional Error
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
        unimplemented!();
    }
}

/// custom type conversion for error handling of Request struct
impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}

pub trait Encrypt {
    fn encrypt(&self) -> Self;
}

impl Encrypt for String {
    fn encrypt(&self) -> Self {
        unimplemented!()
    }
}

impl Encrypt for &[u8] {
    fn encrypt(&self) -> Self {
        unimplemented!()
    }
}