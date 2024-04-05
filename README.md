# SQL minifier
[![Build status](https://github.com/mvisani/SQL-minifier/actions/workflows/build.yml/badge.svg)](https://github.com/mvisani/SQL-minifier/actions)
[![Crates.io](https://img.shields.io/crates/v/SQL-minifier.svg)](https://crates.io/crates/SQL-minifier)
[![Documentation](https://docs.rs/sql_minifier/badge.svg)](https://docs.rs/sql_minifier)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

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
The create provides two main functions:
- `minifiy_sql_to_string` which reads an SQL file and returns a `String` of the
  minified SQL.
- `minifiy_sql_to_file` which reads an SQL file and writes the minified SQL
  to a new file specified by the user.

Additionally, the crate provides a macro `minify_sql_files!` that can be used to minify SQL files at compile time. The macro accepts file paths as input.

It's important to note that the macro will write the minified SQL to a new file with the same name as the input file, but with the suffix `_minified`. Additionally, it will append the `_minified` suffix just before the last `.` in the file name. For instance, if the input file is `test_data/test_file_1.sql`, the minified file will be named `test_data/test_file_1_minified.sql`.

The macro can be utilized as follows:
```rust
use sql_minifier::prelude::*;
minify_sql_files!("test_data/test_file_1.sql", "test_data/test_file_2.sql");
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

## Limitations
The crate currently does not support the removal of SQL `/*`...`*/` multiline comments.