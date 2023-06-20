use std::{fmt, error};

/// Represents an error that can occur while using the `DirWalker` struct.
#[derive(Debug)]
pub enum DirWalkerError {
    /// Indicates an invalid directory path was provided.
    InvalidPath
}

impl fmt::Display for DirWalkerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DirWalkerError::InvalidPath => write!(f, "Error Provide a Directory")
        }
    }
}

impl error::Error for DirWalkerError {}

