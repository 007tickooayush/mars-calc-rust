use crate::method::Method;
use crate::error_parse_request::ParseError;
use crate::model_headers::Headers;
use crate::model_query_string::QueryString;
use crate::model_request_body::RequestBody;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    headers: Headers<'buf>,
    method: Method,
    body: Option<RequestBody>,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    /// NOTE: the expected return type `Option<&QueryString>` is made possible using `.as_ref()` function
    /// else the return type is `&Option<QueryString>` which is not optimal
    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn body(&self) -> Option<&RequestBody> {
        self.body.as_ref()
    }
}

// impl<'buf> Request<'buf> {
//     /// Parse the bytes array from the incoming request and return a Request instance
//     /// and the error message type is Custom, not the traditional Error
//     fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
//         unimplemented!();
//     }
// }

/// Web Request Parsing Utility function
fn get_next_word(req: &str) -> Option<(&str, &str)> {
    for (i, c) in req.chars().enumerate() {
        if c == ' ' || c == '\r' {
            // SUGGEST: Proceeding with the assumption that the provided characters are valid utf8 characters
            // and the code is safe as we are skipping a character ' ' or '\r' and not a byte
            return Some((&req[..i], &req[i + 1..]));
        }
    }
    None
}

/// Web Request Headers Parsing Utility function
fn get_headers(req: &str) -> Option<(&str, &str)> {
    // SUGGEST: skip the first character which is a new line character, for denoting the start of headers
    let iter = req.chars().skip(1);

    for (i, _) in iter.enumerate() {
        if i + 4 < req.len() {
            // print!("{:#?}", c);
            let _r1 = match req.chars().nth(i) {
                Some(m) => m == '\r',
                None => { break }
            };
            let _n1 = match req.chars().nth(i + 1) {
                None => { break }
                Some(n) => n == '\n'
            };

            let _r2 = match req.chars().nth(i + 2) {
                Some(m) => m == '\r',
                None => { break }
            };
            let _n2 = match req.chars().nth(i + 3) {
                None => { break }
                Some(n) => n == '\n'
            };

            // SUGGEST: Checking if the current and next character are new line characters, which would give the request body
            if _r1 && _n1 && _r2 && _n2 {
                // print!("\t\t\t\tCAUGHT");
                return Some((&req[..i], &req[i + 1..]));
            }
        }
    }
    None
}

///

/// custom type conversion for error handling of Request struct
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {

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
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidMethod)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidPath)?;
        let (protocol, request) = get_next_word(request).ok_or(ParseError::InvalidProtocol)?;
        let (headers, request) = get_headers(request).ok_or(ParseError::InvalidRequest)?;
        let body_str = request;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidVersion);
        }

        // SUGGEST: using the ? operator for automatic conversion as the `From` trait is implemented for MethodError in ParseError
        let method: Method = method.parse()?;

        let mut query_string = None;

        // SUGGEST: Keeping the Headers not as Option enum as it is mandatory for the Request struct
        let headers = Headers::from(headers);

        if let Some(idx) = path.find("?") {
            query_string = Some(QueryString::from(&path[idx + 1..]));
            path = &path[..idx];
        }

        // if !body_str.is_empty() {
        //     body = RequestBody::try_from(body_str)
        // }
        let body = match RequestBody::try_from(body_str) {
            Ok(b) => Some(b),
            Err(e) => return Err(ParseError::from(e))
        };

        Ok(Self {
            path,
            query_string,
            headers,
            method,
            body,
        })
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