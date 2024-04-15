#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

pub use minify_sql::minify_sql;

/// This module contains procedural macros
/// to minify SQL queries at compile time
pub mod macros {
    pub use load_sql_proc::load_sql;
    pub use minify_sql_proc::minify_sql;
}
