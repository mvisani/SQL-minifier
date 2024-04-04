/// Implementation of a file parser
/// Each line of the file should be parsed and if the line contains one of the
/// data types that is in a long format, it should replace it by a short format.
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::types::Sqltypes;

/// Parse the file and replace the long format data types by the short format.
pub fn parse_file(file_path: &str) -> Result<String, std::io::Error> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut output = String::new();

    for line in reader.lines() {
        let mut word = String::new();
        let line = line?;
        let mut chars = line.chars();
        let mut comment_started = false;

        while let Some(c) = chars.next() {
            if c == '-' && comment_started {
                // If a comment has started, ignore the rest of the line
                break;
            } else if c == '-' {
                // Check if it's the beginning of a comment
                if let Some(next) = chars.clone().next() {
                    if next == '-' {
                        comment_started = true;
                        continue;
                    }
                }
            }

            if !comment_started {
                if c.is_whitespace() {
                    output.push(c);
                } else {
                    word.push(c);
                    if let Some(next) = chars.clone().next() {
                        if next.is_whitespace() {
                            // If the next character is whitespace, we've collected a word, let's check it
                            match Sqltypes::try_from(word.clone()) {
                                Ok(short) => output.push_str(&format!("{}", short)),
                                Err(_) => output.push_str(&word),
                            }
                            word.clear();
                        }
                    } else {
                        // End of line
                        match Sqltypes::try_from(word.clone()) {
                            Ok(short) => output.push_str(&format!("{}", short)),
                            Err(_) => output.push_str(&word),
                        }
                    }
                }
            }
        }
        output.push('\n');
    }

    Ok(output)
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_file() {
        let file_path = "test_data/test_file_1.txt";
        let expected = "-- Your SQL goes here\nCREATE TABLE IF NOT EXISTS taxa (\n    -- The unique identifier for the taxon\n    id UUID PRIMARY KEY,\n    -- The scientific name of the taxon\n    name TEXT NOT NULL,\n    -- The NCBI Taxon ID is a unique identifier for a taxon in the NCBI Taxonomy database\n    -- which may be NULL when this taxon is not present in the NCBI Taxonomy database.\n    ncbi_taxon_id INT\n);\n";
        let content = parse_file(file_path).unwrap();
        let not_expected = "hello";
        assert_eq!(content, expected);
        assert_ne!(content, not_expected);
    }

    #[test]
    fn test_bigger_file() {
        let file_path = "test_data/test_file_2.txt";
        let expected = "-- SQL to create the projects table.\nCREATE TABLE projects (\n    id UUID PRIMARY KEY,\n    name TEXT NOT NULL UNIQUE,\n    description TEXT NOT NULL,\n    public BOOL NOT NULL DEFAULT TRUE,\n    state_id UUID NOT NULL REFERENCES project_states(id),\n    parent_project_id UUID REFERENCES projects(id) ON\n    DELETE\n        CASCADE,\n        budget FLOAT DEFAULT NULL,\n        expenses FLOAT DEFAULT NULL,\n        created_by UUID NOT NULL REFERENCES users(id),\n        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,\n        expected_end_date TIMESTAMP DEFAULT NULL,\n        end_date TIMESTAMP DEFAULT NULL\n);\n";
        let content = parse_file(file_path).unwrap();

        assert_eq!(content, expected);
    }
}
