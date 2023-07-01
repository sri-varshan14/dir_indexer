use super::{DirTree, DirIndexerErr};
use std::fs;
use std::path::PathBuf;
use std::collections::{HashSet, HashMap};

/// The `DirIndexer` struct represents an indexer for a directory.
///
/// It provides functionality to index files within a directory and retrieve information about them.
#[derive(Debug)]
pub struct DirIndexer {
    root_path_: PathBuf,
    root_tree_: DirTree,
}

impl DirIndexer {
    /// Creates a new `DirIndexer` instance from the specified root path.
    ///
    /// # Arguments
    ///
    /// * `root_path` - The root path of the directory to be indexed.
    ///
    /// # Returns
    ///
    /// An `Option` containing the `DirIndexer` instance if the root path exists and is a directory,
    /// or `None` otherwise.
    pub fn from(root_path: PathBuf) -> Result<DirIndexer,DirIndexerErr> {
        if root_path.exists() && root_path.is_dir() {
            let mut ab_path = root_path.clone();
            if root_path.is_relative() {
                let con_result = fs::canonicalize(root_path.clone());
                if con_result.is_err() {
                    return Err(DirIndexerErr::CanonicalizeFail(root_path));
                }
                else {
                    ab_path = con_result.unwrap(); // No Problem in wrap, I did handle the err
                                                   // before it self
                }
            }
            let dir_tree = DirTree::from(&ab_path);
            Ok(DirIndexer {
                root_path_: ab_path,
                root_tree_: dir_tree,
            })
        } 
        else {
            Err(DirIndexerErr::NotDirNorExist(root_path))
        }
    }

    /// Retrieves a set of relative file paths within the indexed directory and its subdirectories.
    ///
    /// # Returns
    ///
    /// A `HashSet` containing the relative file paths as `PathBuf` values.
    pub fn get_relative_file_paths_set(&self) -> HashSet<PathBuf> {
        self.root_tree_.get_relative_file_paths(&self.root_path_)
    }

    pub fn get_relative_dir_paths_set(&self) -> HashSet<PathBuf> {
        self.root_tree_.get_relative_dir_paths(&self.root_path_)
    }

    /// Retrieves a set of absolute file paths within the indexed directory and its subdirectories.
    ///
    /// # Returns
    ///
    /// A `HashSet` containing the absolute file paths as `PathBuf` values.
    pub fn get_absolute_file_paths_set(&self) -> HashSet<PathBuf> {
        self.root_tree_.get_absolute_file_paths(&self.root_path_)
    }

    pub fn get_absolute_dir_paths_set(&self) -> HashSet<PathBuf> {
        self.root_tree_.get_absolute_dir_paths(&self.root_path_)
    }
    /// Retrieves a mapping between relative and absolute file paths within the indexed directory and its subdirectories.
    ///
    /// # Returns
    ///
    /// A `HashMap` containing the relative file paths as keys and their corresponding absolute file paths as values.
    pub fn get_rl2ab_file_paths_map(&self) -> HashMap<PathBuf, PathBuf> {
        self.root_tree_.get_rl2ab_file_paths(&self.root_path_)
    }

    pub fn get_rl2ab_dir_paths_map(&self) -> HashMap<PathBuf, PathBuf> {
        self.root_tree_.get_rl2ab_dir_paths(&self.root_path_)
    }
    /// Retrieves a mapping between absolute and relative file paths within the indexed directory and its subdirectories.
    ///
    /// # Returns
    ///
    /// A `HashMap` containing the absolute file paths as keys and their corresponding relative file paths as values.
    pub fn get_ab2rl_file_paths_map(&self) -> HashMap<PathBuf, PathBuf> {
        self.root_tree_.get_ab2rl_file_paths(&self.root_path_)
    }

    pub fn get_ab2rl_dir_paths_map(&self) -> HashMap<PathBuf, PathBuf> {
        self.root_tree_.get_ab2rl_dir_paths(&self.root_path_)
    }
}

