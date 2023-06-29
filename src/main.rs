use std::path::PathBuf;

use dir_indexer;

fn main() {
    let path = PathBuf::from("/home/sri/code/dir_indexer/src");
    let a = dir_indexer::DirIndexer::from(path).unwrap();
    let set = a.get_absolute_file_pathss_set();
    for a in set {
        println!("{}",a.to_str().unwrap());
    }
    let set = a.get_relative_file_paths_set();
    for a in set {
        println!("{}",a.to_str().unwrap());
    }
}
