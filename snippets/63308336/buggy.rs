use std::collections::{HashMap};

fn box_and_ref<'a>(map: &'a mut HashMap<String, Box<Vec<u8>>>) -> &'a Vec<u8> {
	let v = vec!{b'h', b'e', b'l', b'l', b'o'};
	let b = Box::new(v);  //line 1
	let r = b.as_ref();   //line 2
	map.insert("foo".to_string(), b); // line 3
	r
}

fn main() {
	let mut map: HashMap<String, Box<Vec<u8>>> = HashMap::new();

	let v = box_and_ref(&mut map);
	println!("{:?}", v);
}
