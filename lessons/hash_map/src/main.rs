use std::collections::HashMap;

fn main() {
    let mut hash_table = HashMap::new();

    // hash_table.insert("demo", 42);
    hash_table.insert("key", 100);

    // let value = hash_table.get("demo").unwrap();

    hash_table.entry("demo").or_insert(100);

    for (k, v) in &hash_table {
        println!("Key: {k}, Value: {v}");
    }
}
