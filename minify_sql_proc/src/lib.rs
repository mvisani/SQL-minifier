//! This crate provides a procedural macro to minify SQL queries at compile time.
#![deny(missing_docs)]
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

/// This macro will minify the provided SQL content at compile time
///
/// # Arguments
/// * `document` - A string slice that holds the SQL content
///
/// # Examples
///
/// ```rust
/// use minify_sql_proc::minify_sql;
///
/// const SQL_CONTENT: &str = minify_sql!(
///     "
/// -- Your SQL goes here
/// CREATE TABLE IF NOT EXISTS taxa (
///    -- The unique identifier for the taxon
///   id UUID PRIMARY KEY,
///   -- The scientific name of the taxon
///  name TEXT NOT NULL,
/// -- The NCBI Taxon ID is a unique identifier for a taxon in the NCBI Taxonomy database
/// -- which may be NULL when this taxon is not present in the NCBI Taxonomy database.
/// ncbi_taxon_id INTEGER
/// );
/// "
/// );
///
/// assert_eq!(
///     SQL_CONTENT,
///     "CREATE TABLE IF NOT EXISTS taxa(id UUID PRIMARY KEY,name TEXT NOT NULL,ncbi_taxon_id INT)"
/// );
/// ```
#[proc_macro]
pub fn minify_sql(input: TokenStream) -> TokenStream {
    // Parse the input token stream
    let document: String = syn::parse_macro_input!(input as syn::LitStr).value();

    // Minify the SQL content
    let minified_document: String = minify_sql::minify_sql(&document);

    // Return the minified SQL content
    TokenStream::from(quote! {
        #minified_document
    })
}
