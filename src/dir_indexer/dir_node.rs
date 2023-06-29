use std::fs;
use std::path::PathBuf;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq )]
pub struct DirNode {
    entry_: PathBuf,
    child_entry_: HashSet<DirNode>
}

impl DirNode {
    pub fn from(root_path: &PathBuf, rl_path: &PathBuf) -> DirNode {
        let joined_path = root_path.join(rl_path);
        let ab_path_entry = fs::canonicalize(joined_path.to_str().unwrap()).unwrap();
        let mut dir_node = DirNode { 
            entry_: rl_path.to_path_buf(), 
            child_entry_: HashSet::new() 
        };

        if ab_path_entry.is_dir() {
            if let Ok(entries) = ab_path_entry.read_dir() {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let entry_name = entry.file_name();
                        let new_entry = rl_path.join(entry_name);
                        dir_node.child_entry_.insert(DirNode::from(root_path, &new_entry));
                    }
                }
            } 
            else {
                println!("Failed to read directory: {:?}", ab_path_entry);
            }
        }
        dir_node
    }
    
    pub fn add_rl_file_path(&self,root_path: &PathBuf, set: &mut HashSet<PathBuf>) {
        let ab_path = fs::canonicalize(root_path.clone().join(self.entry_.clone())).unwrap();
        if ab_path.is_file() {
            set.insert(self.entry_.clone());
        }
        if ab_path.is_dir() {
            for child in &self.child_entry_ {
                child.add_rl_file_path(&root_path, set);
            }
        }
    }

    pub fn add_ab_file_path(&self,root_path: &PathBuf, set: &mut HashSet<PathBuf>) {
        let ab_path = fs::canonicalize(root_path.clone().join(self.entry_.clone())).unwrap();
        if ab_path.is_file() {
            set.insert(ab_path.clone());
        }
        if ab_path.is_dir() {
            for child in &self.child_entry_ {
                child.add_ab_file_path(&root_path, set);
            }
        }
    }

    pub fn relative_entry_name(self) -> PathBuf {
        self.entry_.clone()
    }

    pub fn absolute_entry_name(self, root_path: PathBuf) -> PathBuf {
        let joined_path = root_path.join(self.entry_);
        let ab_path_entry = fs::canonicalize(joined_path.to_str().unwrap()).unwrap();
        return ab_path_entry;
    }

}

impl Eq for DirNode {}

impl Hash for DirNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.entry_.hash(state);
    }
}
