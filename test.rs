use std::collections::HashMap;
fn main() {
    let mut map = HashMap::new();
	map.insert(')','(');
	map.insert(']','[');
	map.insert('}','{');
    println!("Hello, world!  {:?}",map.values());
}
