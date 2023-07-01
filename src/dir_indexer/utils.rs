use std::path::PathBuf;
use std::collections::{HashSet, HashMap};
use super::dir_indexer;

/// Retrieves a set of relative file paths from the specified root path.
///
/// # Arguments
///
/// * `root_path` - The root path of the directory tree.
///
/// # Returns
///
/// A `HashSet` containing the relative file paths.
pub fn get_relative_file_paths_set(root_path: PathBuf) -> HashSet<PathBuf> {
    let dir_idx = dir_indexer::DirIndexer::from(root_path).unwrap();
    dir_idx.get_relative_file_paths_set()
}

pub fn get_relative_dir_paths_set(root_path: PathBuf) -> HashSet<PathBuf> {
    let dir_idx = dir_indexer::DirIndexer::from(root_path).unwrap();
    dir_idx.get_relative_dir_paths_set()
}

/// Retrieves a set of absolute file paths from the specified root path.
///
/// # Arguments
///
/// * `root_path` - The root path of the directory tree.
///
/// # Returns
///
/// A `HashSet` containing the absolute file paths.
pub fn get_absolute_file_paths_set(root_path: PathBuf) -> HashSet<PathBuf> {
    let dir_idx = dir_indexer::DirIndexer::from(root_path).unwrap();
    dir_idx.get_absolute_file_paths_set()
}

pub fn get_absolute_dir_paths_set(root_path: PathBuf) -> HashSet<PathBuf> {
    let dir_idx = dir_indexer::DirIndexer::from(root_path).unwrap();
    dir_idx.get_absolute_dir_paths_set()
}

/// Retrieves a mapping of relative file paths to absolute file paths from the specified root path.
///
/// # Arguments
///
/// * `root_path` - The root path of the directory tree.
///
/// # Returns
///
/// A `HashMap` containing the mapping of relative file paths to absolute file paths.
pub fn get_rl2ab_file_paths_map(root_path: PathBuf) -> HashMap<PathBuf, PathBuf> {
    let dir_idx = dir_indexer::DirIndexer::from(root_path).unwrap();
    dir_idx.get_rl2ab_file_paths_map()
}

pub fn get_rl2ab_dir_paths_map(root_path: PathBuf) -> HashMap<PathBuf, PathBuf> {
    let dir_idx = dir_indexer::DirIndexer::from(root_path).unwrap();
    dir_idx.get_rl2ab_dir_paths_map()
}

/// Retrieves a mapping of absolute file paths to relative file paths from the specified root path.
///
/// # Arguments
///
/// * `root_path` - The root path of the directory tree.
///
/// # Returns
///
/// A `HashMap` containing the mapping of absolute file paths to relative file paths.
pub fn get_ab2rl_file_paths_map(root_path: PathBuf) -> HashMap<PathBuf, PathBuf> {
    let dir_idx = dir_indexer::DirIndexer::from(root_path).unwrap();
    dir_idx.get_ab2rl_file_paths_map()
}

pub fn get_ab2rl_dir_paths_map(root_path: PathBuf) -> HashMap<PathBuf, PathBuf> {
    let dir_idx = dir_indexer::DirIndexer::from(root_path).unwrap();
    dir_idx.get_ab2rl_dir_paths_map()
}

