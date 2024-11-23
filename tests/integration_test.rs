use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::File;
use std::io::Write;
use tempfile::TempDir;

#[test]
fn test_find_string_value_in_object() {
    let mut cmd = Command::cargo_bin("json-key-from-value").unwrap();
    let json_input = r#"{"key1":"value1","key2":"value2"}"#;
    let value_to_find = "\"value1\"";
    cmd.write_stdin(json_input)
        .arg(value_to_find)
        .assert()
        .success()
        .stdout(predicates::str::contains("[\"key1\"]\n"));
}

#[test]
fn test_find_number_value_in_array() {
    let mut cmd = Command::cargo_bin("json-key-from-value").unwrap();
    let json_input = r#"[1, 2, 3, 4]"#;
    let value_to_find = "3";
    cmd.write_stdin(json_input)
        .arg(value_to_find)
        .assert()
        .success()
        .stdout(predicates::str::contains("[2]\n"));
}

#[test]
fn test_find_boolean_value_nested() {
    let mut cmd = Command::cargo_bin("json-key-from-value").unwrap();
    let json_input = r#"{"a": {"b": [true, false]}}"#;
    let value_to_find = "true";
    cmd.write_stdin(json_input)
        .arg(value_to_find)
        .assert()
        .success()
        .stdout(predicates::str::contains("[\"a\"][\"b\"][0]\n"));
}

#[test]
fn test_find_null_value() {
    let mut cmd = Command::cargo_bin("json-key-from-value").unwrap();
    let json_input = r#"{"key": null}"#;
    let value_to_find = "null";
    cmd.write_stdin(json_input)
        .arg(value_to_find)
        .assert()
        .success()
        .stdout(predicates::str::contains("[\"key\"]\n"));
}

#[test]
fn test_multiple_matches() {
    let mut cmd = Command::cargo_bin("json-key-from-value").unwrap();
    let json_input = r#"[1, 2, 3, 2, 1]"#;
    let value_to_find = "1";
    cmd.write_stdin(json_input)
        .arg(value_to_find)
        .assert()
        .success()
        .stdout(predicates::str::contains("[4]\n"))
        .stdout(predicates::str::contains("[0]\n"));
}

#[test]
fn test_value_not_found() {
    let mut cmd = Command::cargo_bin("json-key-from-value").unwrap();
    let json_input = r#"{"key": "value"}"#;
    let value_to_find = "\"nonexistent\"";
    cmd.write_stdin(json_input)
        .arg(value_to_find)
        .assert()
        .success()
        .stdout("");
}

#[test]
fn test_empty_json_object() {
    let mut cmd = Command::cargo_bin("json-key-from-value").unwrap();
    let json_input = r#"{}"#;
    let value_to_find = "1";
    cmd.write_stdin(json_input)
        .arg(value_to_find)
        .assert()
        .success()
        .stdout("");
}

#[test]
fn test_empty_json_array() {
    let mut cmd = Command::cargo_bin("json-key-from-value").unwrap();
    let json_input = r#"[]"#;
    let value_to_find = "1";
    cmd.write_stdin(json_input)
        .arg(value_to_find)
        .assert()
        .success()
        .stdout("");
}

#[test]
fn test_invalid_json_input() {
    let mut cmd = Command::cargo_bin("json-key-from-value").unwrap();
    let json_input = r#"{"key": "value""#; // Missing closing brace
    let value_to_find = "\"value\"";
    cmd.write_stdin(json_input)
        .arg(value_to_find)
        .assert()
        .failure()
        .stderr(predicates::str::contains("Failed to parse input JSON"));
}

#[test]
fn test_invalid_value_input() {
    let mut cmd = Command::cargo_bin("json-key-from-value").unwrap();
    let json_input = r#"{"key": "value"}"#;
    let value_to_find = "\"value"; // Missing closing quote
    cmd.write_stdin(json_input)
        .arg(value_to_find)
        .assert()
        .failure()
        .stderr(predicates::str::contains("Failed to parse search JSON"));
}

#[test]
fn test_reading_from_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.json");
    let mut file = File::create(&file_path).unwrap();
    let json_input = r#"{"file_key": "file_value"}"#;
    write!(file, "{}", json_input).unwrap();

    let mut cmd = Command::cargo_bin("json-key-from-value").unwrap();
    let value_to_find = "\"file_value\"";
    cmd.arg("--json")
        .arg(file_path.to_str().unwrap())
        .arg(value_to_find)
        .assert()
        .success()
        .stdout(predicates::str::contains("[\"file_key\"]\n"));
}

#[test]
fn test_missing_file() {
    let mut cmd = Command::cargo_bin("json-key-from-value").unwrap();
    let value_to_find = "\"value\"";
    cmd.arg("--json")
        .arg("nonexistent.json")
        .arg(value_to_find)
        .assert()
        .failure()
        .stderr(predicates::str::contains("Unable to read file"));
}

#[test]
fn test_complex_nested_structure() {
    let mut cmd = Command::cargo_bin("json-key-from-value").unwrap();
    let json_input = r#"
    {
        "level1": {
            "level2": [
                {
                    "level3": "deep_value"
                }
            ]
        }
    }
    "#;
    let value_to_find = "\"deep_value\"";
    cmd.write_stdin(json_input)
        .arg(value_to_find)
        .assert()
        .success()
        .stdout(predicates::str::contains(
            "[\"level1\"][\"level2\"][0][\"level3\"]\n",
        ));
}

#[test]
fn test_search_for_number_in_string_values() {
    let mut cmd = Command::cargo_bin("json-key-from-value").unwrap();
    let json_input = r#"{"a": "1", "b": 1}"#;
    let value_to_find = "1";
    cmd.write_stdin(json_input)
        .arg(value_to_find)
        .assert()
        .success()
        .stdout(predicates::str::contains("[\"b\"]\n"));
}

#[test]
fn test_search_for_string_number() {
    let mut cmd = Command::cargo_bin("json-key-from-value").unwrap();
    let json_input = r#"{"a": "1", "b": 1}"#;
    let value_to_find = "\"1\"";
    cmd.write_stdin(json_input)
        .arg(value_to_find)
        .assert()
        .success()
        .stdout(predicates::str::contains("[\"a\"]\n"));
}

#[test]
fn test_array_of_objects() {
    let mut cmd = Command::cargo_bin("json-key-from-value").unwrap();
    let json_input = r#"[{"id": 1}, {"id": 2}, {"id": 3}]"#;
    let value_to_find = "2";
    cmd.write_stdin(json_input)
        .arg(value_to_find)
        .assert()
        .success()
        .stdout(predicates::str::contains("[1][\"id\"]\n"));
}

#[test]
fn test_special_characters_in_keys() {
    let mut cmd = Command::cargo_bin("json-key-from-value").unwrap();
    let json_input = r#"{"key-with-dash": "value", "key with space": "value"}"#;
    let value_to_find = "\"value\"";
    cmd.write_stdin(json_input)
        .arg(value_to_find)
        .assert()
        .success()
        .stdout(predicates::str::contains("[\"key-with-dash\"]\n"))
        .stdout(predicates::str::contains("[\"key with space\"]\n"));
}

#[test]
fn test_deeply_nested_arrays() {
    let mut cmd = Command::cargo_bin("json-key-from-value").unwrap();
    let json_input = r#"[ [ [ [ [42] ] ] ] ]"#;
    let value_to_find = "42";
    cmd.write_stdin(json_input)
        .arg(value_to_find)
        .assert()
        .success()
        .stdout(predicates::str::contains("[0][0][0][0][0]\n"));
}

#[test]
fn test_no_arguments() {
    let mut cmd = Command::cargo_bin("json-key-from-value").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Usage"));
}

#[test]
fn test_large_json_input() {
    let mut cmd = Command::cargo_bin("json-key-from-value").unwrap();
    let json_input = format!(
        "[{}]",
        (0..10000)
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
    let value_to_find = "9999";
    cmd.write_stdin(json_input)
        .arg(value_to_find)
        .assert()
        .success()
        .stdout(predicates::str::contains("[9999]\n"));
}

#[test]
fn test_json_with_special_characters() {
    let mut cmd = Command::cargo_bin("json-key-from-value").unwrap();
    let json_input = r#"{"key\nwith\nnewline": "value"}"#;
    let value_to_find = "\"value\"";
    cmd.write_stdin(json_input)
        .arg(value_to_find)
        .assert()
        .success()
        .stdout(predicates::str::contains("[\"key\\nwith\\nnewline\"]\n"));
}

#[test]
fn test_search_for_object_value() {
    let mut cmd = Command::cargo_bin("json-key-from-value").unwrap();
    let json_input = r#"{"a": {"b": 1}, "c": {"b": 1}}"#;
    let value_to_find = r#"{"b":1}"#;
    cmd.write_stdin(json_input)
        .arg(value_to_find)
        .assert()
        .success()
        .stdout(predicates::str::contains("[\"a\"]\n"))
        .stdout(predicates::str::contains("[\"c\"]\n"));
}

#[test]
fn test_search_for_array_value() {
    let mut cmd = Command::cargo_bin("json-key-from-value").unwrap();
    let json_input = r#"{"a": [1,2], "b": [1,2]}"#;
    let value_to_find = "[1,2]";
    cmd.write_stdin(json_input)
        .arg(value_to_find)
        .assert()
        .success()
        .stdout(predicates::str::contains("[\"a\"]\n"))
        .stdout(predicates::str::contains("[\"b\"]\n"));
}

#[test]
fn test_search_with_escaped_characters() {
    let mut cmd = Command::cargo_bin("json-key-from-value").unwrap();
    let json_input = r#"{"quote": "\""}"#;
    let value_to_find = r#""\"""#; // Searching for a double quote character
    cmd.write_stdin(json_input)
        .arg(value_to_find)
        .assert()
        .success()
        .stdout(predicates::str::contains("[\"quote\"]\n"));
}

#[test]
fn test_input_as_command_line_argument() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.json");
    let mut file = File::create(&file_path).unwrap();
    let json_input = r#"{"key": "value"}"#;
    write!(file, "{}", json_input).unwrap();

    let mut cmd = Command::cargo_bin("json-key-from-value").unwrap();
    let value_to_find = "\"value\"";
    cmd.arg("--json")
        .arg(file_path.to_str().unwrap())
        .arg(value_to_find)
        .assert()
        .success()
        .stdout(predicates::str::contains("[\"key\"]\n"));
}

#[test]
fn test_malformed_command_line_json_argument() {
    let mut cmd = Command::cargo_bin("json-key-from-value").unwrap();
    cmd.arg("--json")
        .arg("not_a_json_file")
        .arg("\"value\"")
        .assert()
        .failure()
        .stderr(predicates::str::contains("Unable to read file"));
}

#[test]
fn test_max_results() {
    let mut cmd = Command::cargo_bin("json-key-from-value").unwrap();
    let json_input = r#"[1, 1, 1, 2, 4]"#;
    let value_to_find = "1";
    cmd.write_stdin(json_input)
        .arg(value_to_find)
        .arg("--max-results")
        .arg("2")
        .assert()
        .success()
        .stdout(predicates::str::contains("[0]\n"))
        .stdout(predicates::str::contains("[1]\n"))
        .stdout(predicates::str::contains("[2]\n").not());
}
