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
/// # Arguments
/// * `file_path` - A string slice that holds the path to the file
/// # Returns
/// * A Result containing a string with the minified content of the file
/// # Errors
/// * If the file cannot be opened or read
/// # Examples
/// ```rust
/// use sql_minifier::prelude::*;
/// let file_path = "test_data/test_file_1.sql";
/// let content = minifiy_sql_to_string(file_path).unwrap();
/// ```
pub fn minifiy_sql_to_string(file_path: &str) -> Result<String, std::io::Error> {
    let path = Path::new(file_path);
    let file = File::open(path)?;
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
                    match chars.clone().next() {
                        Some(next) => {
                            if next.is_whitespace() {
                                // If the next character is whitespace, we've collected a word, let's check it
                                match Sqltypes::try_from(word.clone()) {
                                    Ok(short) => output.push_str(&short.to_string()),
                                    Err(_) => output.push_str(&word),
                                }
                                word.clear();
                            }
                        }
                        None => {
                            // End of line
                            match Sqltypes::try_from(word.clone()) {
                                Ok(short) => output.push_str(&short.to_string()),
                                Err(_) => output.push_str(&word),
                            }
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

/// This function will minify SQL file and write the output to a new file
/// # Arguments
/// * `input_file` - A string slice that holds the path to the input file
/// * `output_file` - A string slice that holds the path to the output file
/// # Examples
/// ```rust
/// use sql_minifier::prelude::*;
/// let input_file = "test_data/test_file_1.sql";
/// let output_file = "test_data/test_file_1_minified.sql";
/// minifiy_sql_to_file(input_file, output_file).unwrap();
/// ```
pub fn minifiy_sql_to_file(input_file: &str, output_file: &str) -> Result<(), std::io::Error> {
    let minified_content = minifiy_sql_to_string(input_file)?;
    std::fs::write(output_file, minified_content)?;
    Ok(())
}

/// This macro will minify all the SQL files passed as arguments and save them
/// with the same name but with '_minified.sql' appended to the original filename
/// So if the file name is 'file.sql', the minified file will be 'file_minified.sql'
#[macro_export]
macro_rules! minify_sql_files {
    ($($file:expr),*) => {
        $(
            // output_file should find the last '.sql' and append '_minified.sql' to the filename
            let output_file = {
                let mut output_file = $file.to_string();
                let last_dot = output_file.rfind('.').unwrap();
                output_file.insert_str(last_dot, "_minified");
                output_file
            };
            minifiy_sql_to_file($file, &output_file).unwrap();
        )*
    };
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_file() {
        let file_path = "test_data/test_file_1.sql";
        let expected = "CREATE TABLE IF NOT EXISTS taxa ( id UUID PRIMARY KEY, name TEXT NOT NULL, ncbi_taxon_id INT);";
        let content = minifiy_sql_to_string(file_path).unwrap();
        let not_expected = "hello";
        assert_eq!(content, expected);
        assert_ne!(content, not_expected);
    }

    #[test]
    fn test_bigger_file() {
        let file_path = "test_data/test_file_2.sql";
        let expected = "CREATE TABLE projects ( id UUID PRIMARY KEY, name TEXT NOT NULL UNIQUE, description TEXT NOT NULL, public BOOL NOT NULL DEFAULT TRUE, state_id UUID NOT NULL REFERENCES project_states(id), parent_project_id UUID REFERENCES projects(id) ON DELETE CASCADE, budget FLOAT DEFAULT NULL, expenses FLOAT DEFAULT NULL, created_by UUID NOT NULL REFERENCES users(id), created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, expected_end_date TIMESTAMP DEFAULT NULL, end_date TIMESTAMP DEFAULT NULL);";
        let content = minifiy_sql_to_string(file_path).unwrap();

        assert_eq!(content, expected);
    }

    #[test]
    fn test_file_with_comments() {
        let file_path = "test_data/test_file_3.sql";
        let expected = "CREATE TABLE container_horizontal_rules ( id UUID PRIMARY KEY REFERENCES describables(id) ON DELETE CASCADE, item_type_id UUID REFERENCES item_categories(id) ON DELETE CASCADE, other_item_type_id UUID REFERENCES item_categories(id) ON DELETE CASCADE, minimum_temperature FLOAT DEFAULT NULL, maximum_temperature FLOAT DEFAULT NULL, minimum_humidity FLOAT DEFAULT NULL, maximum_humidity FLOAT DEFAULT NULL, minimum_pressure FLOAT DEFAULT NULL, maximum_pressure FLOAT DEFAULT NULL, CHECK ( minimum_temperature IS NULL OR maximum_temperature IS NULL OR minimum_temperature <= maximum_temperature ), CHECK ( minimum_humidity IS NULL OR maximum_humidity IS NULL OR minimum_humidity <= maximum_humidity ), CHECK ( minimum_pressure IS NULL OR maximum_pressure IS NULL OR minimum_pressure <= maximum_pressure ));";
        let content = minifiy_sql_to_string(file_path).unwrap();

        assert_eq!(content, expected);
    }
}
