# SQL minifier
This crate provides a simple SQL minifier. It removes comments and unnecessary whitespaces from SQL files.

## Usage
```rust
use sql_minifier::prelude::*;
let _ = minify_sql_files!("test_data/test_file_2.sql");
```
