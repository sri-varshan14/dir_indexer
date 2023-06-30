use std::path::PathBuf; 
use std::collections::{HashSet,HashMap};
use super::dir_indexer;

pub fn get_relative_file_paths_set(root_path: PathBuf) -> HashSet<PathBuf> {
    let dir_idx = dir_indexer::DirIndexer::from(root_path).unwrap();
    dir_idx.get_relative_file_paths_set()
}

pub fn get_absolute_file_paths_set(root_path: PathBuf) -> HashSet<PathBuf> {
    let dir_idx = dir_indexer::DirIndexer::from(root_path).unwrap();
    dir_idx.get_absolute_file_pathss_set()
}

pub fn get_rl2ab_file_paths_map(root_path: PathBuf) -> HashMap<PathBuf,PathBuf> {
    let dir_idx = dir_indexer::DirIndexer::from(root_path).unwrap();
    dir_idx.get_rl2ab_file_paths_map()
}

pub fn get_ab2rl_file_paths_map(root_path: PathBuf) -> HashMap<PathBuf,PathBuf> {
    let dir_idx = dir_indexer::DirIndexer::from(root_path).unwrap();
    dir_idx.get_ab2rl_file_paths_map()
}
