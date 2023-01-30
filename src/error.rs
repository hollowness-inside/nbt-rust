use std::{error, fmt, io};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Utf8(std::string::FromUtf8Error),
    UnknownTagType(u8),
    EmptySequence,
    ElementTypesDiffer,
    Generic(String),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::Io(error)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(error: std::string::FromUtf8Error) -> Error {
        Error::Utf8(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(error) => write!(f, "IO error: {error}"),
            Error::Utf8(error) => write!(f, "UTF-8 error: {error}"),
            Error::UnknownTagType(byte) => write!(f, "Unknown tag type: {byte}"),
            Error::EmptySequence => write!(f, "Empty sequence"),
            Error::ElementTypesDiffer => write!(f, "Element types differ"),
            Error::Generic(error) => write!(f, "Generic: {error}"),
        }
    }
}

impl error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
