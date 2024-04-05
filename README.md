# SQL minifier
This crate provides a simple SQL minifier. It removes comments and unnecessary whitespaces from SQL files.

## Installation
Add the following to your `Cargo.toml` file:
```toml
[dependencies]
sql_minifier = "0.1.1"
```

or use the following command:
```bash
cargo add sql_minifier
```

## Usage
```rust
use sql_minifier::prelude::*;
minify_sql_files!("test_data/test_file_1.sql");
```
