# SQL minifier
[![Build status](https://github.com/mvisani/SQL-minifier/actions/workflows/rust.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions)
[![Crates.io](https://img.shields.io/crates/v/SQL-minifier.svg)](https://crates.io/crates/SQL-minifier)
[![Documentation](https://docs.rs/sql_minifier/badge.svg)](https://docs.rs/sql_minifier)

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
