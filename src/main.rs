use clap::Parser;
use serde_json::Value;
use std::fs;
use std::io::{self, Read};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// JSON file to search in. Reads from stdin if not provided.
    #[clap(short, long, value_parser)]
    json: Option<String>,

    /// The value to find
    value: String,
}

fn find_paths(value: &Value, target: &str, current_path: Vec<String>, paths: &mut Vec<String>) {
    match value {
        Value::Object(obj) => {
            for (k, v) in obj {
                let mut path = current_path.clone();
                path.push(format!("[\"{k}\"]"));
                find_paths(v, target, path, paths);
            }
        }
        Value::Array(arr) => {
            for (index, v) in arr.iter().enumerate() {
                let mut path = current_path.clone();
                path.push(format!("[{index}]"));
                find_paths(v, target, path, paths);
            }
        }
        Value::String(s) => {
            if s == target {
                paths.push(current_path.join(""));
            }
        }
        Value::Bool(b) => {
            if let Ok(target_bool) = target.parse::<bool>() {
                if *b == target_bool {
                    paths.push(current_path.join(""));
                }
            }
        }
        Value::Number(num) => {
            if let Ok(target_num) = target.parse::<f64>() {
                if num.as_f64() == Some(target_num) {
                    paths.push(current_path.join(""));
                }
            }
        }
        Value::Null => {
            if target == "null" {
                paths.push(current_path.join(""));
            }
        }
    }
}

fn main() -> serde_json::Result<()> {
    let args = Args::parse();

    let data = match args.json {
        Some(file_path) => fs::read_to_string(file_path).expect("Unable to read file"),
        None => {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .expect("Error reading from stdin");
            buffer
        }
    };

    let json: Value = serde_json::from_str(&data)?;
    let mut paths = Vec::new();

    find_paths(&json, &args.value, vec![], &mut paths);

    for path in paths {
        println!("{path}");
    }

    Ok(())
}
