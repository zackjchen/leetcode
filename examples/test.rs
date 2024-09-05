fn main() {
    let mut map = std::collections::HashMap::new();
    map.entry("k1").or_insert(1);
    map.entry("k1").and_modify(|e| *e -= 1);

    println!("{:?}", map);
}
