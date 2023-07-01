use std::fs;
use std::path::PathBuf;
use std::collections::{HashSet, HashMap};
use std::hash::{Hash, Hasher};

use super::DirIndexerErr;

/// Represents a directory node in the directory tree.
#[derive(Debug, PartialEq)]
pub struct DirNode {
    entry_: PathBuf,
    child_entry_: HashSet<DirNode>,
}

impl DirNode {
    /// Creates a `DirNode` instance from the specified root path and relative path.
    ///
    /// # Arguments
    ///
    /// * `root_path` - The root path of the directory tree.
    /// * `rl_path` - The relative path of the directory node.
    ///
    /// # Returns
    ///
    /// An `Option` containing the `DirNode` instance if successful, or `None` if the path does not exist or is not a directory.
    pub fn from(root_path: &PathBuf, rl_path: &PathBuf) -> Result<DirNode, DirIndexerErr> {
        let joined_path = root_path.join(rl_path);
        if !joined_path.is_file() && !joined_path.is_dir() {
            return Err(DirIndexerErr::NotFileAndDir);
        }
        
        let mut ab_path_entry: PathBuf = PathBuf::new(); 
        let to_str_res = joined_path.to_str();

        if to_str_res.is_some() {
            let con_result = fs::canonicalize(to_str_res.unwrap());
            if con_result.is_ok() {
                ab_path_entry = con_result.unwrap();
            }
        }

        let mut dir_node = DirNode {
            entry_: rl_path.to_path_buf(),
            child_entry_: HashSet::new(),
        };

        if ab_path_entry.is_dir() {
            if let Ok(entries) = ab_path_entry.read_dir() {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let entry_name = entry.file_name();
                        let new_entry = rl_path.join(entry_name);
                        let child_entry = DirNode::from(root_path, &new_entry);

                        if let Ok(a) = child_entry {
                            dir_node.child_entry_.insert(a);
                        }
                    }
                }
            } 
            else {
                return Err(DirIndexerErr::LackPermission(ab_path_entry));
            }
        }
        Ok(dir_node)
    }

    /// Adds relative file paths of the directory node and its children to the provided set.
    ///
    /// # Arguments
    ///
    /// * `root_path` - The root path of the directory tree.
    /// * `set` - The `HashSet` to store the relative file paths.
    pub fn add_rl_file_path(&self, root_path: &PathBuf, set: &mut HashSet<PathBuf>) {
        let ab_path_res = fs::canonicalize(root_path.clone().join(self.entry_.clone()));
        if ab_path_res.is_ok() {
            let ab_path = ab_path_res.unwrap();
            if ab_path.is_file() {
                set.insert(self.entry_.clone());
            }
            if ab_path.is_dir() {
                for child in &self.child_entry_ {
                    child.add_rl_file_path(&root_path, set);
                }
            }
        }
    }

    pub fn add_rl_dir_path(&self, root_path: &PathBuf, set: &mut HashSet<PathBuf>) {
        let ab_path_res = fs::canonicalize(root_path.clone().join(self.entry_.clone()));
        if ab_path_res.is_ok() {
            let ab_path = ab_path_res.unwrap();
            if ab_path.is_dir() {
                set.insert(self.entry_.clone());
                for child in &self.child_entry_ {
                    child.add_rl_dir_path(&root_path, set);
                }
            }
        }
    }

    /// Adds absolute file paths of the directory node and its children to the provided set.
    ///
    /// # Arguments
    ///
    /// * `root_path` - The root path of the directory tree.
    /// * `set` - The `HashSet` to store the absolute file paths.
    pub fn add_ab_file_path(&self, root_path: &PathBuf, set: &mut HashSet<PathBuf>) {
        let ab_path_res = fs::canonicalize(root_path.clone().join(self.entry_.clone()));
        if ab_path_res.is_ok() {
            let ab_path = ab_path_res.unwrap();
            if ab_path.is_file() {
                set.insert(ab_path.clone());
            }
            if ab_path.is_dir() {
                for child in &self.child_entry_ {
                    child.add_ab_file_path(&root_path, set);
                }
            }
        }
    }

    pub fn add_ab_dir_path(&self, root_path: &PathBuf, set: &mut HashSet<PathBuf>) {
        let ab_path_res = fs::canonicalize(root_path.clone().join(self.entry_.clone()));
        if ab_path_res.is_ok() {
            let ab_path = ab_path_res.unwrap();
            if ab_path.is_dir() {
                set.insert(ab_path.clone());
                for child in &self.child_entry_ {
                    child.add_ab_dir_path(&root_path, set);
                }
            }
        }
    }

    /// Maps absolute file paths to relative file paths for the directory node and its children.
    ///
    /// # Arguments
    ///
    /// * `root_path` - The root path of the directory tree.
    /// * `map` - The `HashMap` to store the mapping of absolute file paths to relative file paths.
    pub fn map_ab2rl_file_path(&self, root_path: &PathBuf, map: &mut HashMap<PathBuf, PathBuf>) {
        let ab_path_res = fs::canonicalize(root_path.clone().join(self.entry_.clone()));
        if ab_path_res.is_ok() {
            let ab_path = ab_path_res.unwrap();
            if ab_path.is_file() {
                map.insert(ab_path.clone(), self.entry_.clone());
            }
            if ab_path.is_dir() {
                for child in &self.child_entry_ {
                    child.map_ab2rl_file_path(&root_path, map);
                }
            }
        }
    }

    pub fn map_ab2rl_dir_path(&self, root_path: &PathBuf, map: &mut HashMap<PathBuf, PathBuf>) {
        let ab_path_res = fs::canonicalize(root_path.clone().join(self.entry_.clone()));
        if ab_path_res.is_ok() {
            let ab_path = ab_path_res.unwrap();
            if ab_path.is_dir() {
                map.insert(ab_path.clone(), self.entry_.clone());
                for child in &self.child_entry_ {
                    child.map_ab2rl_dir_path(&root_path, map);
                }
            }
        }
    }

    /// Maps relative file paths to absolute file paths for the directory node and its children.
    ///
    /// # Arguments
    ///
    /// * `root_path` - The root path of the directory tree.
    /// * `map` - The `HashMap` to store the mapping of relative file paths to absolute file paths.
    pub fn map_rl2ab_file_path(&self, root_path: &PathBuf, map: &mut HashMap<PathBuf, PathBuf>) {
        let ab_path_res = fs::canonicalize(root_path.clone().join(self.entry_.clone()));
        if ab_path_res.is_ok() {
            let ab_path = ab_path_res.unwrap();
            if ab_path.is_file() {
                map.insert(self.entry_.clone(), ab_path.clone());
            }
            if ab_path.is_dir() {
                for child in &self.child_entry_ {
                    child.map_ab2rl_file_path(&root_path, map);
                }
            }
        }
    }

    pub fn map_rl2ab_dir_path(&self, root_path: &PathBuf, map: &mut HashMap<PathBuf, PathBuf>) {
        let ab_path_res = fs::canonicalize(root_path.clone().join(self.entry_.clone()));
        if ab_path_res.is_ok() {
            let ab_path = ab_path_res.unwrap();
            if ab_path.is_file() {
                map.insert(self.entry_.clone(), ab_path.clone());
            }
            if ab_path.is_dir() {
                for child in &self.child_entry_ {
                    child.map_ab2rl_dir_path(&root_path, map);
                }
            }
        }
    }

    /// Returns the relative entry name of the directory node.
    pub fn relative_entry_name(self) -> PathBuf {
        self.entry_.clone()
    }

    /// Returns the absolute entry name of the directory node.
    ///
    /// # Arguments
    ///
    /// * `root_path` - The root path of the directory tree.
    ///
    /// # Returns
    ///
    /// The absolute entry name as a `PathBuf`.
    pub fn absolute_entry_name(self, root_path: PathBuf) -> PathBuf {
        let joined_path = root_path.join(self.entry_);
        let ab_path_entry = fs::canonicalize(joined_path.to_str().unwrap()).unwrap();
        return ab_path_entry;
    }
}

impl Eq for DirNode {}

impl Hash for DirNode {
    /// Hashes the `DirNode` using its entry path.
    ///
    /// # Arguments
    ///
    /// * `state` - The `Hasher` to use for hashing.
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.entry_.hash(state);
    }
}

