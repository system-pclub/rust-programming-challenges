extern crate lazy_static;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
	static ref HASHMAP: HashMap<u32, &'static str> = {
		let mut m = HashMap::new();
		m.insert(0, "foo");
		m.insert(1, "bar");
		m.insert(2, "baz");
		m
	};
}

fn main() {
	// First access to `HASHMAP` initializes it
	println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());
	HASHMAP.insert(3, "baz2");
}


