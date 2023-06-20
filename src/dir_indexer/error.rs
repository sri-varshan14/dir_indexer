use std::{fmt, error};

#[derive(Debug)]
pub enum DisWalkerError {
    InvalidPath
}

impl fmt::Display for DisWalkerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DisWalkerError::InvalidPath => write!(f, "Error Provide a Directory")
        }
    }
}

impl error::Error for DisWalkerError {}

