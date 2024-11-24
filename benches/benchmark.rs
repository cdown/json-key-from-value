use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simd_json::to_borrowed_value;
use std::collections::HashMap;

fn benchmark_small_json(c: &mut Criterion) {
    let mut small_json_bytes = r#"{
        "key1": "value1",
        "key2": { "nested_key": "search_value" },
        "key3": ["value2", "value3", { "array_key": "search_value" }]
    }"#
    .to_string()
    .into_bytes();

    let small_json =
        to_borrowed_value(small_json_bytes.as_mut_slice()).expect("Failed to parse small JSON");

    let mut search_value_bytes = b"\"search_value\"".to_vec();
    let search_value =
        to_borrowed_value(search_value_bytes.as_mut_slice()).expect("Failed to parse search value");

    c.bench_function("find_paths_small", |b| {
        b.iter(|| {
            let _ = json_key_from_value::find_paths(
                black_box(&small_json),
                black_box(&search_value),
                black_box(Some(10)),
            );
        });
    });
}

fn benchmark_large_json(c: &mut Criterion) {
    let mut large_json_map = HashMap::new();
    for i in 0..100_000 {
        large_json_map.insert(format!("key{i}"), "value");
    }
    large_json_map.insert("target_key".to_string(), "search_value");

    let mut large_json_bytes = simd_json::to_string(&large_json_map)
        .expect("Failed to serialize large JSON map")
        .into_bytes();
    let large_json =
        to_borrowed_value(large_json_bytes.as_mut_slice()).expect("Failed to parse large JSON");

    let mut search_value_bytes = b"\"search_value\"".to_vec();
    let search_value =
        to_borrowed_value(search_value_bytes.as_mut_slice()).expect("Failed to parse search value");

    c.bench_function("find_paths_large", |b| {
        b.iter(|| {
            let _ = json_key_from_value::find_paths(
                black_box(&large_json),
                black_box(&search_value),
                black_box(Some(10)),
            );
        });
    });
}

criterion_group!(benches, benchmark_small_json, benchmark_large_json);
criterion_main!(benches);
