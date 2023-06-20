/// Node base structure to represent the directory structure
///
/// This module contains the [`DirTreeNode`] type and several methods 
/// related to [`DirTreeNode`] and this is mostly used internally
///
///
use std::path::{Path, PathBuf};
use std::ffi::OsString; 
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

/// Represents a directory tree node.
pub struct DirTreeNode {
    entry_name: OsString,
    is_dir: bool,
    child: HashSet<DirTreeNode>
}

impl Eq for DirTreeNode {}

impl PartialEq for DirTreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.entry_name == other.entry_name && self.is_dir == other.is_dir
    }
}

impl Hash for DirTreeNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.entry_name.hash(state);
        self.is_dir.hash(state)
    }
}

impl DirTreeNode {
    /// Creates a new `DirTreeNode` for the given path.
    ///
    /// If the path represents a directory, the child nodes are recursively populated.
    ///
    /// # Arguments
    ///
    /// * `path` - The path for which to create the `DirTreeNode`.
    ///
    /// # Returns
    ///
    /// A `DirTreeNode` representing the specified path
   pub fn new(path: &Path) -> DirTreeNode {
       let absolute_path = path;
       if path.is_dir() { 
           let mut new_node = DirTreeNode {
               entry_name: absolute_path.file_name().unwrap().into(),
               is_dir: true,
               child: HashSet::new()
           };

           new_node.update_child(absolute_path);
           return new_node;
       }
       DirTreeNode { 
           entry_name: absolute_path.file_name().unwrap().into(), 
           is_dir: false, 
           child: HashSet::new()
       }
   }

   // Updates the child nodes of the current node.
   fn update_child(&mut self, path: &Path){
       let child_entries = path.read_dir().unwrap();
       for entry in child_entries {
           let entry = entry.unwrap();
           let path = entry.path();

           self.child.insert(DirTreeNode::new(&path));
       }
   }

   /// Prints the directory tree structure starting from the current node.
    ///
    /// # Arguments
    ///
    /// * `parent_path` - The parent path used for displaying the tree structure.
   pub fn print(&self, parent_path: &OsString) {
       println!("{}",parent_path.to_str().unwrap());
       if self.is_dir {
            for entry in &self.child {
                let combined_str = OsString::from(
                    format!(
                        "{}/{}",
                        parent_path.to_str().unwrap(),
                        entry.entry_name.to_str().unwrap()
                    ));
                entry.print(&combined_str);
            }
       }
            
   }

    /// Prints the file paths within the directory tree structure starting
    /// from the current node.
    ///
    /// # Arguments
    ///
    /// * `parent_path` - The parent path used for displaying the file paths.
   pub fn print_files(&self, parent_path: &OsString) {
       if self.is_dir {
            for entry in &self.child {
                let combined_str = OsString::from(
                    format!(
                        "{}/{}",
                        parent_path.to_str().unwrap(),
                        entry.entry_name.to_str().unwrap()
                    ));
                entry.print_files(&combined_str);
            }
       }
       else {
           println!("{}",parent_path.to_str().unwrap());
       }
   }
   
    /// Pushes the file paths within the directory tree structure 
    /// into the specified vector.
    ///
    /// # Arguments
    ///
    /// * `parent_path` - The parent path used for building the file paths.
    /// * `vec` - The vector to which the file paths will be pushed.

   pub fn push_file_paths(&self,parent_path: &OsString, vec:&mut  Vec<PathBuf>) {
       if self.is_dir {
            for entry in &self.child {
                let combined_str = OsString::from(
                    format!(
                        "{}/{}",
                        parent_path.to_str().unwrap(),
                        entry.entry_name.to_str().unwrap()
                    ));
                entry.push_file_paths(&combined_str,vec);
            }
       }
       else {
           vec.push(PathBuf::from(parent_path.to_str().unwrap()));
       }
   }

   
    /// Retrieves the total number of files within the directory tree structure
    /// starting from the current node.
    ///
    /// # Returns
    ///
    /// The total number of files.
   pub fn get_number_of_files(&self) -> usize {
        if self.is_dir {
            let mut count_sub_files: usize = 0;
            for entry in &self.child {
                if entry.is_dir {
                    count_sub_files += entry.get_number_of_files()
                }
                else {
                    count_sub_files+=1;
                }
            }
            return count_sub_files;
        }
        else {
            return 1 as usize;
        }
   }

    /// Retrieves the total number of directories within the directory tree
    /// structure starting from the current node.
    ///
    /// # Returns
    ///
    /// The total number of directories.
   pub fn get_number_of_dirs(&self) -> usize {
       let mut count_dirs: usize = 0;
       if self.is_dir {
           count_dirs+=1;
           for entry in &self.child {
               count_dirs += entry.get_number_of_dirs();
           }
       }
       return count_dirs
   }
}
