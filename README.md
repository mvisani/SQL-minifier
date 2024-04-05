# SQL minifier
This crate provides a simple SQL minifier. It removes comments and unnecessary whitespaces from SQL files.

## Usage
```rust
use sql_minifier::prelude::*;
minify_sql_files!("test_data/test_file_1.sql");
```
