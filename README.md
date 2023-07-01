# Directory Indexer

The Directory Indexer is a Rust library that provides functionality for indexing and retrieving information about directory structures. It allows you to obtain sets of relative and absolute file paths, as well as mappings between relative and absolute paths.

## Features

The Directory Indexer library offers the following features:

- **Directory Indexing:** Easily index directory structures and traverse the tree to access file and directory information.

- **Relative and Absolute Paths:** Retrieve sets of relative or absolute file and directory paths from a specified root path.

- **Path Mapping:** Generate mappings between relative and absolute file paths, as well as relative and absolute directory paths.

- **Error Handling:** The library provides error types for handling various scenarios, such as paths that are neither files nor directories or lack of permission to access certain paths.

- **Flexible Usage:** The library is designed to be flexible and can be integrated into different Rust projects.


## Structs

### `DirNode`

The `DirNode` struct represents a node in the directory tree. It has the following fields:

- `entry_` (PathBuf): The relative path of the node.
- `child_entry_` (HashSet<DirNode>): A set of child nodes.

The `DirNode` struct provides methods to traverse and manipulate the directory tree:

- `from(root_path: &PathBuf, rl_path: &PathBuf) -> Result<DirNode, DirIndexerErr>`: Creates a `DirNode` from the specified root path and relative path.
- `add_rl_file_path(&self, root_path: &PathBuf, set: &mut HashSet<PathBuf>)`: Adds relative file paths to the specified set.
- `add_rl_dir_path(&self, root_path: &PathBuf, set: &mut HashSet<PathBuf>)`: Adds relative directory paths to the specified set.
- `add_ab_file_path(&self, root_path: &PathBuf, set: &mut HashSet<PathBuf>)`: Adds absolute file paths to the specified set.
- `add_ab_dir_path(&self, root_path: &PathBuf, set: &mut HashSet<PathBuf>)`: Adds absolute directory paths to the specified set.
- `map_ab2rl_file_path(&self, root_path: &PathBuf, map: &mut HashMap<PathBuf, PathBuf>)`: Maps absolute file paths to relative file paths.
- `map_ab2rl_dir_path(&self, root_path: &PathBuf, map: &mut HashMap<PathBuf, PathBuf>)`: Maps absolute directory paths to relative directory paths.
- `map_rl2ab_file_path(&self, root_path: &PathBuf, map: &mut HashMap<PathBuf, PathBuf>)`: Maps relative file paths to absolute file paths.
- `map_rl2ab_dir_path(&self, root_path: &PathBuf, map: &mut HashMap<PathBuf, PathBuf>)`: Maps relative directory paths to absolute directory paths.
- `relative_entry_name(self) -> PathBuf`: Returns the relative entry name.
- `absolute_entry_name(self, root_path: PathBuf) -> PathBuf`: Returns the absolute entry name.

### `DirIndexerErr`

The `DirIndexerErr` enum represents potential errors that can occur during directory indexing. It has the following variants:

- `NotFileAndDir`: Indicates that the path is neither a file nor a directory.
- `LackPermission(PathBuf)`: Indicates that there is a lack of permission to access the specified path.

## Functions

The Directory Indexer library provides the following functions for easy access to directory indexing functionality:

- `get_relative_file_paths_set(root_path: PathBuf) -> HashSet<PathBuf>`: Retrieves a set of relative file paths from the specified root path.
- `get_relative_dir_paths_set(root_path: PathBuf) -> HashSet<PathBuf>`: Retrieves a set of relative directory paths from the specified root path.
- `get_absolute_file_paths_set(root_path: PathBuf) -> HashSet<PathBuf>`: Retrieves a set of absolute file paths from the specified root path.
- `get_absolute_dir_paths_set(root_path: PathBuf) -> HashSet<PathBuf>`: Retrieves a set of absolute directory paths from the specified root path.
- `get_rl2ab_file_paths_map(root_path: PathBuf) -> HashMap<PathBuf,

 PathBuf>`: Retrieves a mapping of relative file paths to absolute file paths from the specified root path.
- `get_rl2ab_dir_paths_map(root_path: PathBuf) -> HashMap<PathBuf, PathBuf>`: Retrieves a mapping of relative directory paths to absolute directory paths from the specified root path.
- `get_ab2rl_file_paths_map(root_path: PathBuf) -> HashMap<PathBuf, PathBuf>`: Retrieves a mapping of absolute file paths to relative file paths from the specified root path.
- `get_ab2rl_dir_paths_map(root_path: PathBuf) -> HashMap<PathBuf, PathBuf>`: Retrieves a mapping of absolute directory paths to relative directory paths from the specified root path.

## Usage

To use the Directory Indexer library, add it as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
dir_indexer = "0.1.0"
```

Then, import the necessary modules in your Rust code:

```rust
use std::path::PathBuf;
use std::collections::{HashSet, HashMap};
use dir_indexer::{get_relative_file_paths_set, get_absolute_file_paths_set};
```

You can now call the available functions to retrieve directory information:

```rust
fn main() {
    let root_path = PathBuf::from("/path/to/directory");

    // Get a set of relative file paths
    let relative_files = get_relative_file_paths_set(root_path.clone());
    println!("Relative File Paths: {:?}", relative_files);

    // Get a set of absolute file paths
    let absolute_files = get_absolute_file_paths_set(root_path);
    println!("Absolute File Paths: {:?}", absolute_files);
}
```

This example demonstrates how to retrieve a set of relative file paths and a set of absolute file paths from a specified root directory.

## License

This project is licensed under the [MIT License](LICENSE).
