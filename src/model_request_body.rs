use std::fmt::{Debug, Display, Formatter};

type BodyResult = String;

#[derive(Debug)]
/// Using a separate Struct for handling the request body to keep it as a separate entity <br/>
/// The reason is that the request Body can later on be used for REST/SOAP API calls
/// But the length of characters present in the Request body will be limited to 1024 characters
pub struct RequestBody {
    contents: BodyResult,
}

impl RequestBody {
    pub fn contents(&self) -> BodyResult {
        self.contents.clone()
    }
}

impl TryFrom<&str> for RequestBody {
    type Error = RequestBodyError;

    fn try_from(value: &str) -> Result<RequestBody, RequestBodyError> {
        let mut res = String::new();
        if value.len() < 1024 {
            for c in value.chars() {
                if c == '\r' || c == '\n' || c == '\t' || c == ' ' || c == '\0' || c == '\u{0}' {
                    continue;
                }
                else if c.is_alphanumeric() || c.is_ascii_punctuation() {
                    res.push(c);
                } else {
                    return Err(RequestBodyError::InvalidCharacters);
                }
            }

            Ok(RequestBody {
                contents: res,
            })
        } else {
            Err(RequestBodyError::MaxLengthExceeded)
        }
    }
}

pub enum RequestBodyError {
    MaxLengthExceeded,
    InvalidCharacters,
}

impl Debug for RequestBodyError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Display for RequestBodyError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl RequestBodyError {
    fn message(&self) -> &str {
        match self {
            Self::MaxLengthExceeded => "Max length exceeded",
            Self::InvalidCharacters => "Invalid characters",
        }
    }
}