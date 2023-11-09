use std::io;

#[derive(Debug)]
pub enum ParseFileError {
    InvalidRead(io::Error),
    InvalidLine(String),
}

impl From<io::Error> for ParseFileError {
    fn from(error: io::Error) -> Self {
        ParseFileError::InvalidRead(error)
    }
}

impl From<String> for ParseFileError {
    fn from(error: String) -> Self {
        ParseFileError::InvalidLine(error)
    }
}

