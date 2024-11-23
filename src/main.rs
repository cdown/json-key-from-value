use anyhow::{Context, Result};
use clap::Parser;
use serde_json::Value;
use std::fs;
use std::io::{self, Read};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// JSON file to search in. Reads from stdin if not provided.
    #[clap(short, long)]
    json: Option<String>,

    /// The value to find.
    value: String,
}

fn find_paths(
    value: &Value,
    target: &Value,
    current_path: &mut Vec<String>,
    paths: &mut Vec<String>,
) {
    if value == target {
        paths.push(current_path.join(""));
    }

    match value {
        Value::Object(map) => {
            for (k, v) in map {
                current_path.push(format!("[\"{k}\"]"));
                find_paths(v, target, current_path, paths);
                current_path.pop();
            }
        }
        Value::Array(arr) => {
            for (i, v) in arr.iter().enumerate() {
                current_path.push(format!("[{i}]"));
                find_paths(v, target, current_path, paths);
                current_path.pop();
            }
        }
        _ => {}
    }
}

fn main() -> Result<()> {
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

    let json: Value = serde_json::from_str(&data).context("Failed to parse input JSON")?;
    let target_value = serde_json::from_str(&args.value).context("Failed to parse search JSON")?;

    let mut paths = Vec::new();
    let mut current_path = Vec::new();

    find_paths(&json, &target_value, &mut current_path, &mut paths);

    for path in paths {
        println!("{}", path);
    }

    Ok(())
}
