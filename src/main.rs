use std::path::{Path, PathBuf};

use dir_indexer;

fn main() {
    let path = Path::new("/home/sri/code");
    let a = dir_indexer::DirTreeNode::new(path);
    let path = PathBuf::from("/home/sri/code/sri-com-svelte/build/");
    let a = dir_indexer::DirWalker::new(&path).unwrap();
    println!("{:#?}",a.files_list());
}
