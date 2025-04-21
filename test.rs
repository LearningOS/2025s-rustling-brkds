use std::collections::HashMap;
fn main() {
	let some_number: Option<i32> = Some(5);
	let none_number: Option<i32> = None;
	
	println!("{}", some_number.unwrap_or(0));  // 输出 5
	println!("{}", none_number.unwrap_or(0));   // 输出 0
}
