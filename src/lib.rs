use anyhow::Result;
use simd_json::borrowed::Value;
use std::borrow::Cow;

struct StackItem<'json> {
    value: &'json Value<'json>,
    depth: usize,
    path_element: Option<JsonPath<'json>>,
}

#[derive(Clone, PartialEq, Eq)]
enum JsonPath<'json> {
    Key(&'json str),
    Index(usize),
}

impl JsonPath<'_> {
    fn format(&self) -> String {
        match self {
            JsonPath::Key(key) => format!(
                "[{}]",
                simd_json::to_string(&Value::String(Cow::Borrowed(key))).expect("invalid key")
            ),
            JsonPath::Index(index) => format!("[{index}]"),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum SearchType {
    Value,
    Key,
}

pub fn find_paths(
    value: &Value,
    target: &Value,
    max_results: Option<usize>,
    search_type: SearchType,
) -> Result<Vec<String>> {
    let mut stack = vec![StackItem {
        value,
        depth: 0,
        path_element: None,
    }];
    let mut paths = Vec::new();
    let mut path = Vec::new();
    let key_target = match (&search_type, target) {
        (SearchType::Key, Value::String(s)) => Some(s),
        (SearchType::Key, _) => {
            anyhow::bail!("Cannot use key mode with non string target: {target:?}")
        }
        _ => None,
    };

    while let Some(StackItem {
        value,
        depth,
        path_element,
    }) = stack.pop()
    {
        let found = if search_type == SearchType::Key {
            path_element == Some(JsonPath::Key(key_target.unwrap()))
        } else {
            value == target
        };

        path.truncate(depth);
        if let Some(elem) = path_element {
            path.push(elem);
        }

        if found {
            paths.push(path.iter().map(JsonPath::format).collect());
            if max_results.map_or(false, |max| paths.len() == max) {
                break;
            }

            if search_type == SearchType::Value {
                // If we are looking for values they are absolute and non-recursive, but this is
                // not the case for keys, which can be seen further down within the tree
                continue;
            }
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
