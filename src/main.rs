use std::path::PathBuf;

use dir_indexer;

fn main() {
    let path = PathBuf::from("/home/sri");
    let map = dir_indexer::get_rl2ab_file_paths_map(path);
    for (k,v) in map {
        println!("{} -> {}",k.to_str().unwrap(),v.to_str().unwrap());
    }
}
