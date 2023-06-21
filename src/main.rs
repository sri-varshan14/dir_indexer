use std::path::PathBuf;

use dir_indexer;
fn main() {
    let path = PathBuf::from("src");
    let a = dir_indexer::DirWalker::new(&path).unwrap();
    println!("{:#?}",a.files_list());
    println!("{:#?}",a.directory_list());
    println!("{:#?}",a.ndirs());
    println!("{}",a.nfiles());
}
