use anyhow::Result;
use serde_json::Value;

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

pub fn find_paths(
    value: &Value,
    target: &Value,
    max_results: Option<usize>,
) -> Result<Vec<String>> {
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
                stack.extend(map.iter().map(|(k, v)| StackItem {
                    value: v,
                    depth: depth + 1,
                    path_element: Some(JsonPath::Key(k)),
                }));
            }
            Value::Array(arr) => {
                // We pop from the back, so put final indices on first, otherwise the interaction
                // with --max-results is weird.
                stack.extend(arr.iter().enumerate().rev().map(|(i, v)| StackItem {
                    value: v,
                    depth: depth + 1,
                    path_element: Some(JsonPath::Index(i)),
                }));
            }
            _ => {}
        }
    }

    Ok(paths)
}
