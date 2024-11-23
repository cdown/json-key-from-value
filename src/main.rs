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

struct StackItem<'json> {
    value: &'json Value,
    path: Vec<String>,
}

fn find_paths(value: &Value, target: &Value) -> Vec<String> {
    let mut stack = vec![StackItem {
        value,
        path: Vec::new(),
    }];
    let mut paths = Vec::new();

    while let Some(StackItem { value, path }) = stack.pop() {
        if value == target {
            paths.push(path.join(""));
        }

        match value {
            Value::Object(map) => {
                for (k, value) in map {
                    let mut new_path = path.clone();
                    new_path.push(format!("[\"{k}\"]"));
                    stack.push(StackItem {
                        value,
                        path: new_path,
                    });
                }
            }
            Value::Array(arr) => {
                for (i, value) in arr.iter().enumerate() {
                    let mut new_path = path.clone();
                    new_path.push(format!("[{i}]"));
                    stack.push(StackItem {
                        value,
                        path: new_path,
                    });
                }
            }
            _ => {}
        }
    }

    paths
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

    let paths = find_paths(&json, &target_value);

    for path in paths {
        println!("{}", path);
    }

    Ok(())
}
