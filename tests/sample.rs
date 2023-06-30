use std::path::PathBuf;
use dir_indexer::DirIndexer;
use std::any::TypeId;

#[test]
fn test_creating_dir_indexer_obj() {
    let a = DirIndexer::from(PathBuf::from("source"));
    assert_ne!(a.is_some(), true);
}
