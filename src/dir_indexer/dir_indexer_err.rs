use std::fmt;
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug)]
pub enum DirIndexerErr {
    CanonicalizeFail(PathBuf),
    NotDirNorExist(PathBuf),
    LackPermission(PathBuf),
    NotFileAndDir,
    ToStrFail,
    TreeConstructFailed
}

impl fmt::Display for DirIndexerErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self{
            DirIndexerErr::CanonicalizeFail(a) => write!(f, "{} unable to use fs::canonicalize()\n Reasons:\n Path does not exist", a.to_str().unwrap()),
            DirIndexerErr::NotDirNorExist(a) => write!(f, "{} path does not exist", a.to_str().unwrap()),
            DirIndexerErr::LackPermission(a) => write!(f, "{} User Lack Permission Over the Directory", a.to_str().unwrap()),
            DirIndexerErr::NotFileAndDir => write!(f, ""),
            DirIndexerErr::ToStrFail => write!(f, ""),
            DirIndexerErr::TreeConstructFailed => write!(f, ""),

        }
    }
}

impl Error for DirIndexerErr {}
