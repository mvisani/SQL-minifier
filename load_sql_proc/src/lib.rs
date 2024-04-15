//! This crate provides a procedural macro to minify SQL queries at compile time.
#![deny(missing_docs)]
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

/// This macro will load and minify the provided SQL document at compile time
///
/// # Arguments
/// * `path` - A string slice that holds the path to the SQL file
///
/// # Examples
///
/// ```rust
/// use load_sql_proc::load_sql;
///
/// const SQL_CONTENT: &str = load_sql!("tests/test_file_1.sql");
///
/// assert_eq!(
///     SQL_CONTENT,
///     "CREATE TABLE IF NOT EXISTS taxa(id UUID PRIMARY KEY,name TEXT NOT NULL,ncbi_taxon_id INT)"
/// );
/// ```
#[proc_macro]
pub fn load_sql(input: TokenStream) -> TokenStream {
    // Parse the input token stream
    let path = syn::parse_macro_input!(input as syn::LitStr).value();

    // We prepend CARGO_HOME to the path, as the path is relative to the project root
    let path = format!("{}/{}", std::env::var("CARGO_MANIFEST_DIR").unwrap(), path);

    // Read the content of the file
    let document = std::fs::read_to_string(path).expect("Could not read SQL file to minify");

    // Minify the SQL content
    let minified_document: String = minify_sql::minify_sql(&document);

    // Return the minified SQL content
    TokenStream::from(quote! {
        #minified_document
    })
}
