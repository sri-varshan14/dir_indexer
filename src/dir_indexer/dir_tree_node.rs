use std::{path::{Path, PathBuf}, ffi::OsString, str::FromStr };

pub struct DirTreeNode {
    entry_name: OsString,
    is_dir: bool,
    child: Vec<DirTreeNode>
}

impl DirTreeNode {
   pub fn new(path: &Path) -> DirTreeNode {
       let absolute_path = path;
       if path.is_dir() { 
           let mut new_node = DirTreeNode {
               entry_name: absolute_path.file_name().unwrap().into(),
               is_dir: true,
               child: Vec::new()
           };

           new_node.update_child(absolute_path);
           return new_node;
       }
       DirTreeNode { 
           entry_name: absolute_path.file_name().unwrap().into(), 
           is_dir: false, 
           child: Vec::new() 
       }
   }

   pub fn update_child(&mut self, path: &Path){
       let child_entries = path.read_dir().unwrap();
       for entry in child_entries {
           let entry = entry.unwrap();
           let path = entry.path();

           self.child.push(DirTreeNode::new(&path))
       }
   }

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
}
