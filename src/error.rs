use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::Utf8Error;

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
    InvalidPath,
    InvalidVersion,
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result { // different type of Result struct
        write!(f, "{}", self.message())
    }
}

/// Whenever from_utf8() function fails, it will return ParseError::InvalidEncoding
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Error for ParseError {}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid request", // ParseError::InvalidRequest
            Self::InvalidEncoding => "Invalid encoding", // ParseError::InvalidEncoding
            Self::InvalidProtocol => "Invalid protocol", // ParseError::InvalidProtocol
            Self::InvalidMethod => "Invalid method", // ParseError::InvalidMethod
            Self::InvalidPath => "Invalid path", // ParseError::InvalidPath
            Self::InvalidVersion => "Invalid version", // ParseError::InvalidVersion
        }
    }
}