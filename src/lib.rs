//! This crate provides bindings for the Sirius executable.
//! All the parameters of sirius should be accessible from this crate.
#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod file_parser;
mod types;

/// Prelude module
pub mod prelude {
    pub use crate::types::*;
}
