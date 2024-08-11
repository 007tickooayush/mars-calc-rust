use std::fmt::{Debug, Display, Formatter};

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