# json-key-from-value | [![Tests](https://img.shields.io/github/actions/workflow/status/cdown/json-key-from-value/ci.yml?branch=master)](https://github.com/cdown/json-key-from-value/actions?query=branch%3Amaster)

If you often have big, unwieldy blobs of JSON which you have very deep trees
in, it can be a pain to go from a value that you know is the field you want to
retrieve to actually producing the correct selector.

json-key-from-value will, given some JSON and a value to find, tell you how to
select it. If multiple keys or indexes are possible, they will all be printed.

## Usage

For example, given the following JSON:

```
{"data":[{"foo":[
    {"bar":[
        {"id":1,"name":"yes"},
        {"id":2,"name":"no"}
    ]},
    {"baz":[
        {"id":2,"name":null}
    ]}]
},{"qux":[
    {"quux":
        [[1,42],[2,2],[true, false]]
    }]
}]}
```

    % json-key-from-value '"yes"' < json
    ["data"][0]["foo"][0]["bar"][0]["name"]

    % json-key-from-value 2 < json
    ["data"][0]["foo"][0]["bar"][1]["id"]
    ["data"][0]["foo"][1]["baz"][0]["id"]
    ["data"][1]["qux"][0]["quux"][1][0]
    ["data"][1]["qux"][0]["quux"][1][1]

    % json-key-from-value true < json
    ["data"][1]["qux"][0]["quux"][2][0]

    % json-key-from-value null < json
    ["data"][0]["foo"][1]["baz"][0]["name"]

## Installation

    cargo install json-key-from-value
