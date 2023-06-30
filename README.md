# Directory Indexer

The Directory Indexer is a Rust library that allows you to index and retrieve file paths within a directory tree. It provides convenient methods to obtain relative and absolute file paths, as well as mappings between relative and absolute paths.

## Features

- Retrieve a set of relative file paths from a specified root directory.
- Retrieve a set of absolute file paths from a specified root directory.
- Get a mapping of relative file paths to their corresponding absolute file paths.
- Get a mapping of absolute file paths to their corresponding relative file paths.

## Installation

Add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
dir_indexer = "0.0.1"
```

## Usage

1. Import the necessary modules and types:

```rust
use std::path::PathBuf;
use std::collections::{HashSet, HashMap};
use dir_indexer::{DirIndexer, get_relative_file_paths_set, get_absolute_file_paths_set, get_rl2ab_file_paths_map, get_ab2rl_file_paths_map};
```

2. Create a `DirIndexer` instance by providing the root directory path:

```rust
let root_path = PathBuf::from("/path/to/root/directory");
let dir_indexer = DirIndexer::from(root_path).expect("Failed to create DirIndexer");
```

3. Use the available methods to retrieve the desired file path information:

```rust
// Retrieve a set of relative file paths
let relative_paths: HashSet<PathBuf> = dir_indexer.get_relative_file_paths_set();

// Retrieve a set of absolute file paths
let absolute_paths: HashSet<PathBuf> = dir_indexer.get_absolute_file_paths_set();

// Get a mapping of relative file paths to absolute file paths
let rl2ab_map: HashMap<PathBuf, PathBuf> = dir_indexer.get_rl2ab_file_paths_map();

// Get a mapping of absolute file paths to relative file paths
let ab2rl_map: HashMap<PathBuf, PathBuf> = dir_indexer.get_ab2rl_file_paths_map();
```

Alternatively, you can use the provided utility functions for a simplified interface:

```rust
let root_path = PathBuf::from("/path/to/root/directory");

// Retrieve a set of relative file paths using the utility function
let relative_paths: HashSet<PathBuf> = get_relative_file_paths_set(root_path.clone());

// Retrieve a set of absolute file paths using the utility function
let absolute_paths: HashSet<PathBuf> = get_absolute_file_paths_set(root_path.clone());

// Get a mapping of relative file paths to absolute file paths using the utility function
let rl2ab_map: HashMap<PathBuf, PathBuf> = get_rl2ab_file_paths_map(root_path.clone());

// Get a mapping of absolute file paths to relative file paths using the utility function
let ab2rl_map: HashMap<PathBuf, PathBuf> = get_ab2rl_file_paths_map(root_path.clone());
```

## License

This project is licensed under the [MIT License](LICENSE).

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvement, please open an issue or submit a pull request.

## Acknowledgements

- The Directory Indexer library is inspired by the need for efficient directory indexing and file path retrieval in Rust projects.

## Contact

For any inquiries or questions, feel free to contact the project maintainer at [mailmeatsri14@gmail.com](mailto:mailmeatsri14@gmail.com).

---
