use std::{io, fmt, error};

use serde::{ser, de};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Serde
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::Io(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(error) => write!(f, "IO error: {}", error),
            Error::Serde => write!(f, "Serde error"),
        }
    }
}

impl error::Error for Error {}


impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Serde
    }
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Serde
    }
}

pub type Result<T> = std::result::Result<T, Error>;
