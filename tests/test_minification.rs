//! This file contains tests for the minification of SQL files.
use sql_minifier::macros::load_sql;
use std::fs;

#[test]
fn test_parse_file() {
    let expected_file_path = "tests/test_file_1_minified.sql";
    let expected = fs::read_to_string(expected_file_path).unwrap();
    let content = load_sql!("tests/test_file_1.sql");
    let not_expected = "hello";
    assert_eq!(content, expected);
    assert_ne!(content, not_expected);
}

#[test]
fn test_bigger_file() {
    let expected_file_path = "tests/test_file_2_minified.sql";
    let expected = fs::read_to_string(expected_file_path).unwrap();
    let content = load_sql!("tests/test_file_2.sql");

    assert_eq!(content, expected);
}

#[test]
fn test_file_with_comments() {
    let expected_file_path = "tests/test_file_3_minified.sql";
    let expected = fs::read_to_string(expected_file_path).unwrap();
    let content = load_sql!("tests/test_file_3.sql");

    assert_eq!(content, expected);
}

#[test]
fn test_balanced_parentheses() {
    let expected_file_path = "tests/test_balanced_multiline_comments_minified.sql";
    let expected = fs::read_to_string(expected_file_path).unwrap();
    let content = load_sql!("tests/test_balanced_multiline_comments.sql");

    assert_eq!(content, expected);
}

#[test]
fn test_balanced_parentheses2() {
    let expected_file_path = "tests/test_balanced_comment2_minified.sql";
    let expected = fs::read_to_string(expected_file_path).unwrap();
    let content = load_sql!("tests/test_balanced_comment2.sql");

    assert_eq!(content, expected);
}

#[test]
fn test_table_name_with_type() {
    let expected_file_path = "tests/test_table_name_with_type_minified.sql";
    let expected = fs::read_to_string(expected_file_path).unwrap();
    let content = load_sql!("tests/test_table_name_with_type.sql");

    assert_eq!(content, expected);
}
