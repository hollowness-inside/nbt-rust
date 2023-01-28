use std::{error, fmt, io};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Serde,
    Generic(String),
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
            Error::Generic(error) => write!(f, "Generic: {}", error),
            Error::Serde => write!(f, "Serde"),
        }
    }
}

impl error::Error for Error {}
impl serde::ser::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: fmt::Display,
    {
        Error::Serde
    }
}

pub type Result<T> = std::result::Result<T, Error>;
