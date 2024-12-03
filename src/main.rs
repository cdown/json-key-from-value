use anyhow::{Context, Result};
use clap::Parser;
use json_key_from_value::{find_paths, SearchType};
use simd_json::to_borrowed_value;
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

    /// Search for a key instead of a value.
    #[clap(short, long)]
    key: bool,

    /// The value to find.
    value: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Reading from stdin is unbounded in time, validate the bounded part first
    let mut value_bytes = args.value.into_bytes();
    let target_value =
        to_borrowed_value(&mut value_bytes).context("Failed to parse search JSON")?;

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
    let mut data_bytes = data.into_bytes();
    let json = to_borrowed_value(&mut data_bytes).context("Failed to parse input JSON")?;

    let search_type = if args.key {
        SearchType::Key
    } else {
        SearchType::Value
    };

    let paths = find_paths(&json, &target_value, args.max_results, search_type)?;

    for path in paths {
        println!("{}", path);
    }

    Ok(())
}
