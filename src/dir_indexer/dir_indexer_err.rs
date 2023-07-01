use std::fmt;
use std::error::Error;
use std::path::PathBuf;

/// Represents possible errors that can occur during directory indexing.
#[derive(Debug)]
pub enum DirIndexerErr {
    /// Failed to canonicalize the path.
    CanonicalizeFail(PathBuf),
    /// The path does not exist or is not a directory.
    NotDirNorExist(PathBuf),
    /// User lacks permission over the directory.
    LackPermission(PathBuf),
    /// The path is neither a file nor a directory.
    NotFileAndDir,
    /// Failed to convert path to a string.
    ToStrFail,
    /// Failed to construct the directory tree.
    TreeConstructFailed,
}

impl fmt::Display for DirIndexerErr {
    /// Formats the error message for display.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DirIndexerErr::CanonicalizeFail(path) => {
                write!(f, "{} unable to use fs::canonicalize()\n Reasons:\n Path does not exist", path.to_str().unwrap())
            }
            DirIndexerErr::NotDirNorExist(path) => {
                write!(f, "{} path does not exist", path.to_str().unwrap())
            }
            DirIndexerErr::LackPermission(path) => {
                write!(f, "{} User Lack Permission Over the Directory", path.to_str().unwrap())
            }
            DirIndexerErr::NotFileAndDir => {
                write!(f, "The provided path is neither a file nor a directory")
            }
            DirIndexerErr::ToStrFail => {
                write!(f, "Failed to convert path to string")
            }
            DirIndexerErr::TreeConstructFailed => {
                write!(f, "Failed to construct directory tree")
            }
        }
    }
}

impl Error for DirIndexerErr {}

