//! Directory Walker that traverse the Directory and files
//! to generate a tree like structure using [`DirTreeNode`]
//! representing the new Node\Entries from the Directory
//!
//! Note: Only Files and Directory are validate not symbolic link 

use crate::DirTreeNode;
use crate::DirWalkerError;
use std::ffi::OsString;
use std::path::PathBuf;
use std::path::Path;

/// Provides a directory walker to traverse and analyze directory structures.
pub struct DirWalker {
    root_directory: OsString,
    tree: DirTreeNode,
    no_of_files: usize,
    no_of_dirs: usize
}

impl DirWalker {
    /// Creates a new `DirWalker` for the specified path.
    ///
    /// The `DirWalker` recursively builds a directory tree structure starting
    /// from the specified path.
    /// It also updates the count of files and directories within the tree.
    ///
    /// # Arguments
    ///
    /// * `path` - The path representing the root directory for the `DirWalker`.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `DirWalker` if successful, or a `DirWalkerError` if the path is invalid.
    pub fn new(path: &PathBuf) -> Result<DirWalker, DirWalkerError> {
        if path.is_dir() {
            let mut dir_walker_obj =  DirWalker{ 
                root_directory: path.as_os_str().to_os_string(), 
                tree: DirTreeNode::new(path),
                no_of_files:0,
                no_of_dirs: 0,
            };
            dir_walker_obj.update_dir_number();
            dir_walker_obj.update_file_number();
            return Ok(dir_walker_obj);
        }
        Err(DirWalkerError::InvalidPath)
    }

    /// Prints the directory tree structure starting from the root directory.
    pub fn print(&self) {
        self.tree.print(&OsString::new())
    }

    /// Prints the file paths within the directory tree structure starting
    /// from the root directory.
    pub fn print_files(&self) {
        self.tree.print_files(&OsString::new())
    }

    /// Retrieves a list of file paths within the directory tree structure
    /// starting from the root directory.
    ///
    /// # Returns
    ///
    /// A vector containing the file paths.
    pub fn files_list(&self) -> Vec<PathBuf> {
        let mut file_vec = Vec::new();
        self.tree.push_file_paths(&OsString::new(),&mut file_vec);
        file_vec
    }

    /// Updates the count of files within the directory tree structure.
    pub fn update_file_number(&mut self) {
        self.no_of_files = self.tree.get_number_of_files()
    }

    /// Retrieves the count of files within the directory tree structure.
    ///
    /// # Returns
    ///
    /// The count of files.
    pub fn nfiles(&self) -> usize{
        self.no_of_files
    }

    // Updates the count of directories within the directory tree structure.
    pub fn update_dir_number(&mut self) {
        self.no_of_dirs = self.tree.get_number_of_dirs()
    }

    /// Retrieves the count of directories within the directory tree structure.
    ///
    /// # Returns
    ///
    /// The count of directories.
    pub fn ndirs(&self) -> usize{
        self.no_of_dirs
    }

    /// Updates the directory tree structure and counts by rebuilding
    /// it based on the root directory.
    pub fn update_tree(&mut self) {
       let path = Path::new(&self.root_directory);
       self.tree = DirTreeNode::new(path);
       self.update_file_number();
       self.update_dir_number();
    }
}
