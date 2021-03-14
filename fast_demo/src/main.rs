use std::error::Error;
use std::fmt::{Display, Formatter, Debug, Pointer};
use std::num::ParseIntError;
use std::str::Utf8Error;

#[derive(Debug)]
enum CustomError {
    ParseIntError(std::num::ParseIntError),
    Utf8Error(std::str::Utf8Error),
    IoError(std::io::Error),
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            CustomError::ParseIntError(e) => { Pointer::fmt( &e,f) }
            CustomError::Utf8Error(   e) => {Pointer::fmt(&e,f) }
            CustomError::IoError(   e) => { Pointer::fmt(&e,f) }
        }
    }
}


impl Error for CustomError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            CustomError::ParseIntError(ref e) => { Some(e) }
            CustomError::Utf8Error(e) => { Some(e) }
            CustomError::IoError(e) => { Some(e) }
        }
    }
}

impl From<ParseIntError> for CustomError {
    fn from(parse: ParseIntError) -> Self {
        CustomError::ParseIntError(parse)
    }
}

impl From<std::io::Error> for CustomError {
    fn from(stdError: std::io::Error) -> Self {
        CustomError::IoError(stdError)
    }
}

impl From<Utf8Error> for CustomError {
    fn from(utf8Error: Utf8Error) -> Self {
        CustomError::Utf8Error(utf8Error)
    }
}


fn main() {}