use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use super::DirTree;


#[derive(Debug)]
pub struct DirIndexer {
    root_path_: PathBuf,
    root_tree_: DirTree
}

impl DirIndexer {
    pub fn from(root_path: PathBuf) -> Option<DirIndexer> {
        if root_path.exists() && root_path.is_dir() {
            let ab_path = root_path.clone();
            if root_path.is_relative() {
                let ab_path = fs::canonicalize(root_path).expect("Failed to get absolute path.");
            }
            let dir_tree = DirTree::from(&ab_path);
            Some(
                DirIndexer {
                    root_path_: ab_path,
                    root_tree_: dir_tree
                }
            )
        }
        else {
            None
        }
    }

    pub fn get_relative_file_paths_set(&self) -> HashSet<PathBuf> {
        self.root_tree_.get_relative_file_paths(&self.root_path_)
    }

    pub fn get_absolute_file_pathss_set(&self) -> HashSet<PathBuf> {
        self.root_tree_.get_absolute_file_paths(&self.root_path_)
    }
}
