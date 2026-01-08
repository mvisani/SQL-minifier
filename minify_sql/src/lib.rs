//! Crate providing a function to minify SQL content.
#![deny(missing_docs)]

use regex::Regex;

/// Returns the provided SQL content minified.
///
/// # Arguments
/// * `path` - A string slice that holds the path to the SQL file
///
/// # Examples
///
/// ```rust
/// use minify_sql::minify_sql;
///
/// let minified: String = minify_sql(
///     "-- Your SQL goes here
/// CREATE TABLE IF NOT EXISTS taxa (
///     -- The unique identifier for the taxon
///     id UUID PRIMARY KEY,
///     -- The scientific name of the taxon
///     name TEXT NOT NULL,
///     -- The NCBI Taxon ID is a unique identifier for a taxon in the NCBI Taxonomy database
///     -- which may be NULL when this taxon is not present in the NCBI Taxonomy database.
///     ncbi_taxon_id INTEGER
/// );
/// ",
/// );
///
/// assert_eq!(
///     minified,
///     "CREATE TABLE IF NOT EXISTS taxa(id UUID PRIMARY KEY,name TEXT NOT NULL,ncbi_taxon_id INT)"
/// );
/// ```
pub fn minify_sql(document: &str) -> String {
    // Preliminarly, we remove all multiline comments from the file
    // We need to this first, as the multiline comments can span multiple lines
    // and if we remove the line breaks first, we might accidentally add new
    // combinations of characters that seem to be an open or close comment.
    let document_without_multiline_comments = remove_multiline_comments(document);

    // We remove in all lines of the file the single line comments
    let mut document_without_comments =
        remove_single_line_comments(&document_without_multiline_comments);

    // We apply the minifications relative to the SQL types, such as replacing
    // "INTEGER" by "INT", "BOOLEAN" by "BOOL", "CHARACTER" by "CHAR", and
    // "DECIMAL" by "DEC", while handling the case where table names of column
    // names contain these words.

    for (long, short) in LONG_FORMAT_TYPES {
        let re = Regex::new(&format!(r"\b{}\b", long)).unwrap();
        document_without_comments = re
            .replace_all(&document_without_comments, short)
            .to_string();
    }

    // remove all excess whitespaces meaning that if the string has more that
    // one whitespace, it will be replaced by a single whitespace
    let mut output = document_without_comments
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    // Remove all whitespace before and after commas, semicolons, and parentheses (either
    // opening or closing), as well as before or after operators
    for symbols in vec![
        ",", ";", "(", ")", ">", "<", ">=", "<=", "!=", "<>", "=", "+", "-", "*", "/",
    ] {
        output = output.replace(&format!(" {}", symbols), symbols);
        output = output.replace(&format!("{} ", symbols), symbols);
    }

    // If the last character is a semi-colon, remove it, as it is not needed when executing
    // the SQL statement. It would be solely needed when executing multiple statements in a row
    if output.ends_with(';') {
        output.pop();
    }

    output
}

#[cfg(all(not(feature = "gluesql"), not(feature = "sqlite")))]
/// List of long format data types and their corresponding short format
const LONG_FORMAT_TYPES: [(&str, &str); 5] = [
    ("INTEGER", "INT"),
    ("BOOLEAN", "BOOL"),
    ("CHARACTER", "CHAR"),
    ("DECIMAL", "DEC"),
    ("TEMPORARY", "TEMP"),
];

#[cfg(feature = "sqlite")]
/// List of long format data types and their corresponding short format
const LONG_FORMAT_TYPES: [(&str, &str); 4] = [
    ("BOOLEAN", "BOOL"),
    ("CHARACTER", "CHAR"),
    ("DECIMAL", "DEC"),
    ("TEMPORARY", "TEMP"),
];

#[cfg(feature = "gluesql")]
/// List of long format data types and their corresponding short format
/// for GlueSQL
const LONG_FORMAT_TYPES: [(&str, &str); 4] = [
    ("INTEGER", "INT"),
    ("CHARACTER", "CHAR"),
    ("DECIMAL", "DEC"),
    ("TEMPORARY", "TEMP"),
];

/// Remove all multiline comments from the SQL content
///
/// # Arguments
/// * `sql_content` - A string slice that holds the content of the SQL file
fn remove_multiline_comments(sql_content: &str) -> String {
    // A multiline comment is a classical example of balanced parenthesis.
    // We can use this to our advantage to remove them from the SQL content,
    // where the parenthesis in question is \* and *\. These are two characters
    // and not one, so we need to keep track of the last two characters we've seen
    // to determine if we're in a comment or not.
    let mut output = String::new();

    let mut last_char = char::default();
    let mut number_of_open_comments: u32 = 0;

    for mut c in sql_content.chars() {
        if number_of_open_comments > 0 && last_char == '*' && c == '/' {
            // We're closing a comment
            number_of_open_comments -= 1;
            c = char::default();
        } else if last_char == '/' && c == '*' {
            // We're opening a comment
            number_of_open_comments += 1;
            c = char::default();
        } else if number_of_open_comments == 0 {
            // Maybe we are not in a comment
            if c != '/' {
                // We're not in a comment
                if last_char == '/' {
                    output.push('/');
                }
                output.push(c);
            }
        }
        last_char = c;
    }

    output
}

/// Remove all single line comments from the SQL content
///
/// # Arguments
/// * `document` - A string slice that holds the content of the SQL file
fn remove_single_line_comments(document: &str) -> String {
    let mut output = String::new();

    // Once we detect a single line comment, we can ignore the rest of the line
    // and continue with the next line. In SQL, a single line comment is denoted
    // by two dashes "--" and goes until the end of the line.
    for line in document.lines() {
        let mut last_char = char::default();

        for c in line.chars() {
            if last_char == '-' && c == '-' {
                // We're starting a comment
                output.pop();
                break;
            }

            output.push(c);

            last_char = c;
        }

        // Add a space to separate the lines
        output.push(' ');
    }

    output
}
