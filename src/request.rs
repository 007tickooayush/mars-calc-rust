use crate::method::Method;
use std::convert;
use crate::error::ParseError;

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

/// Web Request Parsing Utility function
fn get_next_word(req: &str) -> Option<(&str, &str)> {
    let mut iter = req.chars();
    for (i, c) in req.chars().enumerate() {
        if c == ' '  || c == '\r' {
            // SUGGEST: Proceeding with the assumption that the provided characters are valid utf8 characters
            // and the code is safe as we are skipping a character ' ' or '\r' and not a byte
            return Some((&req[..i], &req[i + 1..]));
        }
    }
    None
}

/// custom type conversion for error handling of Request struct
impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {

        // SUGGEST: long implementation
        // match std::str::from_utf8(buf) {
        //     Ok(request) => {
        //         unimplemented!()
        //     },
        //     Err(_) => {
        //         return Err(ParseError::InvalidEncoding)
        //     }
        // }

        // SUGGEST: using ? operator and or function to handle and return custom error
        // let result = std::str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;

        // SUGGEST: Shortest error handling using ? operator and implementing the `From` trait for Utf8Error
        let request = std::str::from_utf8(buf)?;

        // match get_next_word(request) {
        //     Some((method, request)) => {
        //         unimplemented!()
        //     },
        //     None => {
        //         return Err(ParseError::InvalidRequest)
        //     }
        // }

        // SYNTAX: .ok_or(ParseError::InvalidRequest)
        // SUGGEST: using the ok_or function to handle the error
        // the function returns first word and rest of the string slice
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidVersion);
        }

        // SUGGEST: using the ? operator for automatic conversion as the `From` trait is implemented for MethodError in ParseError
        let method: Method = method.parse()?;

        let mut query_str = None;

        if let Some (idx) = path.find("?") {
            query_str = Some(&path[idx+1..]);
            path = &path[..idx];
        }


        unimplemented!()
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