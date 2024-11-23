use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde_json::json;

fn benchmark_small_json(c: &mut Criterion) {
    let small_json = json!({
        "key1": "value1",
        "key2": { "nested_key": "search_value" },
        "key3": ["value2", "value3", { "array_key": "search_value" }]
    });

    c.bench_function("find_paths_small", |b| {
        b.iter(|| {
            let _ = json_key_from_value::find_paths(
                black_box(&small_json),
                black_box(&json!("search_value")),
                black_box(Some(10)),
            );
        });
    });
}

fn benchmark_large_json(c: &mut Criterion) {
    let mut large_json = serde_json::Map::new();
    for i in 0..100_000 {
        large_json.insert(format!("key{i}"), json!("value"));
    }
    large_json.insert("target_key".to_string(), json!("search_value"));
    let large_json = serde_json::Value::Object(large_json);

    c.bench_function("find_paths_large", |b| {
        b.iter(|| {
            let _ = json_key_from_value::find_paths(
                black_box(&large_json),
                black_box(&json!("search_value")),
                black_box(Some(10)),
            );
        });
    });
}

criterion_group!(benches, benchmark_small_json, benchmark_large_json);
criterion_main!(benches);
