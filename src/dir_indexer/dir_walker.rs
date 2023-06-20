use crate::DirTreeNode;
use crate::DisWalkerError;
use std::ffi::OsString;
use std::path::{PathBuf,Path};

#[warn(dead_code)]
pub struct DirWalker {
    root_directory: OsString,
    tree: DirTreeNode
}

impl DirWalker {
    pub fn new(path: &PathBuf) -> Result<DirWalker, DisWalkerError> {
        if path.is_dir() {
            return Ok( 
                DirWalker{ 
                    root_directory: path.as_os_str().to_os_string(), 
                    tree: DirTreeNode::new(path) 
                })
        }
        Err(DisWalkerError::InvalidPath)
    }

    pub fn print(&self) {
        self.tree.print(&OsString::new())
    }

    pub fn print_files(&self) {
        self.tree.print_files(&OsString::new())
    }

    pub fn files_list(&self) -> Vec<PathBuf> {
        let mut file_vec = Vec::new();
        self.tree.push_file_paths(&OsString::new(),&mut file_vec);
        file_vec
    }
}
