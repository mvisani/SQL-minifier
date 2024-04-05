//! This crate provides bindings for the Sirius executable.
//! All the parameters of sirius should be accessible from this crate.
#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod minifier;
mod types;

/// Prelude module
pub mod prelude {
    pub use crate::minifier::{minifiy_sql_to_file, minifiy_sql_to_string};
    pub use crate::minify_sql_files;
}
