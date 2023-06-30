use super::DirNode;
use std::path::PathBuf;
use std::collections::{HashSet, HashMap};

#[derive(Debug)]
pub struct DirTree {
    root_node_: DirNode
}

impl DirTree {
    pub fn from(ab_path: &PathBuf) -> DirTree {
        let relative_start = PathBuf::from("./");
        let node = DirNode::from(ab_path, &relative_start).unwrap();
        DirTree { root_node_: node }
    }

    pub fn get_relative_file_paths(&self, root_path: &PathBuf) -> HashSet<PathBuf> {
        let mut path_set: HashSet<PathBuf> = HashSet::new();
        self.root_node_.add_rl_file_path(&root_path , &mut path_set);
        path_set
    }

    pub fn get_absolute_file_paths(&self, root_path: &PathBuf) -> HashSet<PathBuf> {
        let mut path_set: HashSet<PathBuf> = HashSet::new();
        self.root_node_.add_ab_file_path(root_path , &mut path_set);
        path_set
    }

    pub fn get_rl2ab_file_paths(&self, root_path: &PathBuf) -> HashMap<PathBuf,PathBuf> {
        let mut path_map: HashMap<PathBuf,PathBuf> = HashMap::new();
        self.root_node_.map_rl2ab_file_path(root_path,&mut path_map);
        path_map
    }

    pub fn get_ab2rl_file_paths(&self, root_path: &PathBuf) -> HashMap<PathBuf,PathBuf> {
        let mut path_map: HashMap<PathBuf,PathBuf> = HashMap::new();
        self.root_node_.map_ab2rl_file_path(root_path,&mut path_map);
        path_map
    }
}
