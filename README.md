# SQL minifier
[![Build status](https://github.com/mvisani/SQL-minifier/actions/workflows/build.yml/badge.svg)](https://github.com/mvisani/SQL-minifier/actions)
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

## Example
The following SQL file:
```sql
-- Your SQL goes here
CREATE TABLE IF NOT EXISTS taxa (
    -- The unique identifier for the taxon
    id UUID PRIMARY KEY,
    -- The scientific name of the taxon
    name TEXT NOT NULL,
    -- The NCBI Taxon ID is a unique identifier for a taxon in the NCBI Taxonomy database
    -- which may be NULL when this taxon is not present in the NCBI Taxonomy database.
    ncbi_taxon_id INTEGER
);
```

will be minified to:
```sql
CREATE TABLE IF NOT EXISTS taxa ( id UUID PRIMARY KEY, name TEXT NOT NULL, ncbi_taxon_id INT);
```