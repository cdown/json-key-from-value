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

    /// Stop searching after this many paths are found.
    #[clap(short, long)]
    max_results: Option<usize>,

    /// The value to find.
    value: String,
}

struct StackItem<'json> {
    value: &'json Value,
    depth: usize,
    path_element: Option<JsonPath<'json>>,
}

#[derive(Clone)]
enum JsonPath<'json> {
    Key(&'json String),
    Index(usize),
}

impl JsonPath<'_> {
    fn format(&self) -> String {
        match self {
            JsonPath::Key(key) => format!("[{}]", serde_json::to_string(key).expect("invalid key")),
            JsonPath::Index(index) => format!("[{index}]"),
        }
    }
}

fn find_paths(value: &Value, target: &Value, max_results: Option<usize>) -> Result<Vec<String>> {
    let mut stack = vec![StackItem {
        value,
        depth: 0,
        path_element: None,
    }];
    let mut paths = Vec::new();
    let mut path = Vec::new();

    while let Some(StackItem {
        value,
        depth,
        path_element,
    }) = stack.pop()
    {
        path.truncate(depth);
        if let Some(elem) = path_element {
            path.push(elem);
        }

        if value == target {
            paths.push(path.iter().map(JsonPath::format).collect());
            if max_results.map_or(false, |max| paths.len() == max) {
                break;
            }
            continue;
        }

        match value {
            Value::Object(map) => {
                for (k, value) in map {
                    stack.push(StackItem {
                        value,
                        depth: path.len(),
                        path_element: Some(JsonPath::Key(k)),
                    });
                }
            }
            Value::Array(arr) => {
                // We pop from the back, so put final indices on first, otherwise the interaction
                // with --max-results is weird.
                for (i, value) in arr.iter().enumerate().rev() {
                    stack.push(StackItem {
                        value,
                        depth: path.len(),
                        path_element: Some(JsonPath::Index(i)),
                    });
                }
            }
            _ => {}
        }
    }

    Ok(paths)
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

    let json = serde_json::from_str(&data).context("Failed to parse input JSON")?;
    let target_value = serde_json::from_str(&args.value).context("Failed to parse search JSON")?;

    let paths = find_paths(&json, &target_value, args.max_results)?;

    for path in paths {
        println!("{}", path);
    }

    Ok(())
}
