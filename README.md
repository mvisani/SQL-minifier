# SQL minifier
[![Build status](https://github.com/mvisani/SQL-minifier/actions/workflows/build.yml/badge.svg)](https://github.com/mvisani/SQL-minifier/actions)
[![Crates.io](https://img.shields.io/crates/v/SQL-minifier.svg)](https://crates.io/crates/SQL-minifier)
[![Documentation](https://docs.rs/sql_minifier/badge.svg)](https://docs.rs/sql_minifier)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

[SQL minifier](https://github.com/mvisani/SQL-minifier) provides methods and procedural macros to minify SQL code, optionally at compile time.
It removes both single-line `--` and multi-line `/* ... */` comments, unnecessary whitespaces, and shortens SQL keywords such as `INTEGER` to `INT`.

## Installation
Add the following to your `Cargo.toml` file:
```toml
[dependencies]
sql_minifier = "0.1.6"
```

or use the following command:
```bash
cargo add sql_minifier
```

## Examples
Suppose you have an SQL string and you want to minify it. You can use the [`minify_sql`](https://github.com/mvisani/SQL-minifier/blob/b22627d27905ea5ce92d3a683ac10d11bccbfae9/minify_sql/src/lib.rs#L35) function:

```rust
use sql_minifier::minify_sql;

let minified: String = minify_sql(
    "-- Your SQL goes here
CREATE TABLE IF NOT EXISTS taxa (
    -- The unique identifier for the taxon
    id UUID PRIMARY KEY,
    -- The scientific name of the taxon
    name TEXT NOT NULL,
    -- The NCBI Taxon ID is a unique identifier for a taxon in the NCBI Taxonomy database
    -- which may be NULL when this taxon is not present in the NCBI Taxonomy database.
    ncbi_taxon_id INTEGER
);"
);

assert_eq!(
    minified,
    "CREATE TABLE IF NOT EXISTS taxa(id UUID PRIMARY KEY,name TEXT NOT NULL,ncbi_taxon_id INT)"
);
```

If you want this to be done at compile time, you can use the [`minify_sql`](https://github.com/mvisani/SQL-minifier/blob/b22627d27905ea5ce92d3a683ac10d11bccbfae9/minify_sql_proc/src/lib.rs#L39) macro:
```rust
use sql_minifier::macros::minify_sql;

const SQL_CONTENT: &str = minify_sql!(
    "-- Your SQL goes here
CREATE TABLE IF NOT EXISTS taxa (
    -- The unique identifier for the taxon
    id UUID PRIMARY KEY,
    -- The scientific name of the taxon
    name TEXT NOT NULL,
    -- The NCBI Taxon ID is a unique identifier for a taxon in the NCBI Taxonomy database
    -- which may be NULL when this taxon is not present in the NCBI Taxonomy database.
    ncbi_taxon_id INTEGER
);"
);

assert_eq!(
    SQL_CONTENT,
    "CREATE TABLE IF NOT EXISTS taxa(id UUID PRIMARY KEY,name TEXT NOT NULL,ncbi_taxon_id INT)"
);
```

A more complex [SQL file](tests/test_file_3.sql) such as:
```sql
-- SQL defining the container_horizontal_rules table.
-- The container horizontal rules define whether an item type can be placed next to another item type.
-- For instance a acid product cannot be placed next to a base product. Generally speaking, most items
-- can be placed next to each other, but some items cannot be placed next to each other. These rules
-- are defined in the form of a deny-list, meaning that if a rule is not defined, then the item type
-- can be placed next to any other item type. The rules are defined by an admin user, and are used to
-- enforce the placement rules when creating or updating items. Some items may only be placed next to
-- items that are within a certain temperature, humidity, or pressure range. These constraints are also
-- defined in the container rules.
CREATE TABLE container_horizontal_rules (
    id UUID PRIMARY KEY REFERENCES describables(id) ON DELETE CASCADE,
    item_type_id UUID REFERENCES item_categories(id) ON
    DELETE
        CASCADE,
        other_item_type_id UUID REFERENCES item_categories(id) ON
    DELETE
        CASCADE,
        minimum_temperature FLOAT DEFAULT NULL,
        maximum_temperature FLOAT DEFAULT NULL,
        minimum_humidity FLOAT DEFAULT NULL,
        maximum_humidity FLOAT DEFAULT NULL,
        minimum_pressure FLOAT DEFAULT NULL,
        maximum_pressure FLOAT DEFAULT NULL,
        CHECK (
            minimum_temperature IS NULL
            OR maximum_temperature IS NULL
            OR minimum_temperature <= maximum_temperature
        ),
        /* The minimum humidity must be less than or
        equal to the maximum humidity. */
        CHECK (
            minimum_humidity IS NULL
            OR maximum_humidity IS NULL
            OR minimum_humidity <= maximum_humidity
        ),
        CHECK (
            minimum_pressure IS NULL
            OR maximum_pressure IS NULL
            OR minimum_pressure <= maximum_pressure
        )
);
/* and other multiline comment */
```

We can load it and minify it at compile time using the [`load_sql`](https://github.com/mvisani/SQL-minifier/blob/b22627d27905ea5ce92d3a683ac10d11bccbfae9/load_sql_proc/src/lib.rs#L26) macro:
```rust
use sql_minifier::macros::load_sql;

const SQL_CONTENT: &str = load_sql!("tests/test_file_3.sql");

assert_eq!(
    SQL_CONTENT,
    "CREATE TABLE container_horizontal_rules(id UUID PRIMARY KEY REFERENCES describables(id)ON DELETE CASCADE,item_type_id UUID REFERENCES item_categories(id)ON DELETE CASCADE,other_item_type_id UUID REFERENCES item_categories(id)ON DELETE CASCADE,minimum_temperature FLOAT DEFAULT NULL,maximum_temperature FLOAT DEFAULT NULL,minimum_humidity FLOAT DEFAULT NULL,maximum_humidity FLOAT DEFAULT NULL,minimum_pressure FLOAT DEFAULT NULL,maximum_pressure FLOAT DEFAULT NULL,CHECK(minimum_temperature IS NULL OR maximum_temperature IS NULL OR minimum_temperature<=maximum_temperature),CHECK(minimum_humidity IS NULL OR maximum_humidity IS NULL OR minimum_humidity<=maximum_humidity),CHECK(minimum_pressure IS NULL OR maximum_pressure IS NULL OR minimum_pressure<=maximum_pressure))"
);
```

## Features
We support the following features:
- `gluesql`: When enabled, the minifier will not minify BOOLEAN keywords to BOOL, as it is not supported by [GlueSQL](https://gluesql.org/docs/0.14/).
- `sqlite` : When enabled, the minifier will not minify INTEGER keywords to INT, when AUTOINCREMENT is present.