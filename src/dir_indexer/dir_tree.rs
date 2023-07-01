use super::DirNode;
use std::path::PathBuf;
use std::collections::{HashSet, HashMap};

/// The `DirTree` struct represents a tree structure of a directory.
///
/// It provides operations to retrieve file paths and mappings between file paths within the directory tree.
#[derive(Debug)]
pub struct DirTree {
    root_node_: DirNode,
}

impl DirTree {
    /// Creates a new `DirTree` instance from the specified absolute path.
    ///
    /// # Arguments
    ///
    /// * `ab_path` - The absolute path of the root directory.
    ///
    /// # Returns
    ///
    /// A `DirTree` instance.
    pub fn from(ab_path: &PathBuf) -> DirTree {
        let relative_start = PathBuf::from("");
        let node = DirNode::from(ab_path, &relative_start).unwrap(); // May be I need to conside to
                                                                     // handle this function but
                                                                     // lets this in future
        DirTree { root_node_: node }
    }

    /// Retrieves a set of relative file paths within the directory tree.
    ///
    /// # Arguments
    ///
    /// * `root_path` - The root path of the directory tree.
    ///
    /// # Returns
    ///
    /// A `HashSet` containing the relative file paths as `PathBuf` values.
    pub fn get_relative_file_paths(&self, root_path: &PathBuf) -> HashSet<PathBuf> {
        let mut path_set: HashSet<PathBuf> = HashSet::new();
        self.root_node_.add_rl_file_path(&root_path, &mut path_set);
        path_set
    }

    pub fn get_relative_dir_paths(&self, root_path: &PathBuf) -> HashSet<PathBuf> {
        let mut path_set: HashSet<PathBuf> = HashSet::new();
        self.root_node_.add_rl_dir_path(&root_path, &mut path_set);
        path_set
    }

    /// Retrieves a set of absolute file paths within the directory tree.
    ///
    /// # Arguments
    ///
    /// * `root_path` - The root path of the directory tree.
    ///
    /// # Returns
    ///
    /// A `HashSet` containing the absolute file paths as `PathBuf` values.
    pub fn get_absolute_file_paths(&self, root_path: &PathBuf) -> HashSet<PathBuf> {
        let mut path_set: HashSet<PathBuf> = HashSet::new();
        self.root_node_.add_ab_file_path(root_path, &mut path_set);
        path_set
    }

    pub fn get_absolute_dir_paths(&self, root_path: &PathBuf) -> HashSet<PathBuf> {
        let mut path_set: HashSet<PathBuf> = HashSet::new();
        self.root_node_.add_ab_dir_path(root_path, &mut path_set);
        path_set
    }

    /// Retrieves a mapping between relative and absolute file paths within the directory tree.
    ///tree
    /// # Arguments
    ///
    /// * `root_path` - The root path of the directory tree.
    ///
    /// # Returns
    ///
    /// A `HashMap` containing the relative file paths as keys and their corresponding absolute file paths as values.
    pub fn get_rl2ab_file_paths(&self, root_path: &PathBuf) -> HashMap<PathBuf, PathBuf> {
        let mut path_map: HashMap<PathBuf, PathBuf> = HashMap::new();
        self.root_node_.map_rl2ab_file_path(root_path, &mut path_map);
        path_map
    }

    pub fn get_rl2ab_dir_paths(&self, root_path: &PathBuf) -> HashMap<PathBuf, PathBuf> {
        let mut path_map: HashMap<PathBuf, PathBuf> = HashMap::new();
        self.root_node_.map_rl2ab_dir_path(root_path, &mut path_map);
        path_map
    }
    /// Retrieves a mapping between absolute and relative file paths within the directory tree.
    ///
    /// # Arguments
    ///
    /// * `root_path` - The root path of the directory tree.
    ///
    /// # Returns
    ///
    /// A `HashMap` containing the absolute file paths as keys and their corresponding relative file paths as values.
    pub fn get_ab2rl_file_paths(&self, root_path: &PathBuf) -> HashMap<PathBuf, PathBuf> {
        let mut path_map: HashMap<PathBuf, PathBuf> = HashMap::new();
        self.root_node_.map_ab2rl_file_path(root_path, &mut path_map);
        path_map
    }

    pub fn get_ab2rl_dir_paths(&self, root_path: &PathBuf) -> HashMap<PathBuf, PathBuf> {
        let mut path_map: HashMap<PathBuf, PathBuf> = HashMap::new();
        self.root_node_.map_ab2rl_dir_path(root_path, &mut path_map);
        path_map
    }
}

