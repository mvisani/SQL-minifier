use crate::types::Sqltypes;
/// Implementation of a file parser
/// Each line of the file should be parsed and if the line contains one of the
/// data types that is in a long format, it should replace it by a short format.
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Parse the file and replace the long format data types by the short format,
///  while ignoring SQL comments.
/// The function should read line by line the file, split at whitespaces and
/// convert long format data types to short format.
pub fn minifiy_sql_file(file_path: &str) -> Result<String, std::io::Error> {
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
                            Err(_) => output.push_str(&format!("{}", word)),
                        }
                    }
                }
            }
        }
    }
    // remove all excess whitespaces meaning that if the string has more that
    // one whitespace, it will be replaced by a single whitespace
    let output = output.split_whitespace().collect::<Vec<&str>>().join(" ");
    Ok(output)
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_file() {
        let file_path = "test_data/test_file_1.sql";
        let expected = "CREATE TABLE IF NOT EXISTS taxa ( id UUID PRIMARY KEY, name TEXT NOT NULL, ncbi_taxon_id INT);";
        let content = minifiy_sql_file(file_path).unwrap();
        let not_expected = "hello";
        assert_eq!(content, expected);
        assert_ne!(content, not_expected);
    }

    #[test]
    fn test_bigger_file() {
        let file_path = "test_data/test_file_2.sql";
        let expected = "CREATE TABLE projects ( id UUID PRIMARY KEY, name TEXT NOT NULL UNIQUE, description TEXT NOT NULL, public BOOL NOT NULL DEFAULT TRUE, state_id UUID NOT NULL REFERENCES project_states(id), parent_project_id UUID REFERENCES projects(id) ON DELETE CASCADE, budget FLOAT DEFAULT NULL, expenses FLOAT DEFAULT NULL, created_by UUID NOT NULL REFERENCES users(id), created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, expected_end_date TIMESTAMP DEFAULT NULL, end_date TIMESTAMP DEFAULT NULL);";
        let content = minifiy_sql_file(file_path).unwrap();

        assert_eq!(content, expected);
    }
}
