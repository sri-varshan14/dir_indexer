use std::path::PathBuf;

use dir_indexer;

fn main() {
    let path = PathBuf::from("/home/sri/code/dir_indexer/src");
    println!("");
    for (k,v) in dir_indexer::get_ab2rl_file_paths_map(path.clone()) {
        println!("{} -> {}", k.to_str().unwrap(), v.to_str().unwrap());
    }
    println!("");
    for (k,v) in dir_indexer::get_rl2ab_file_paths_map(path.clone()) {
        println!("{} -> {}", k.to_str().unwrap(), v.to_str().unwrap());
    }
    println!("");
    for v in dir_indexer::get_absolute_file_paths_set(path.clone()) {
        println!("{}", v.to_str().unwrap());
    }
    println!("");
    for v in dir_indexer::get_relative_file_paths_set(path.clone()) {
        println!("{}", v.to_str().unwrap());
    }
    println!("");
    for (k,v) in dir_indexer::get_ab2rl_dir_paths_map(path.clone()) {
        println!("{} -> {}", k.to_str().unwrap(), v.to_str().unwrap());
    }
    println!("");
    for v in dir_indexer::get_absolute_dir_paths_set(path.clone()) {
        println!("{}", v.to_str().unwrap());
    }
    println!("");
    for (k,v) in dir_indexer::get_rl2ab_dir_paths_map(path.clone()) {
        println!("{} -> {}", k.to_str().unwrap(), v.to_str().unwrap());
    }
    println!("");
    for v in dir_indexer::get_relative_dir_paths_set(path.clone()) {
        println!("{}", v.to_str().unwrap());
    }
}
