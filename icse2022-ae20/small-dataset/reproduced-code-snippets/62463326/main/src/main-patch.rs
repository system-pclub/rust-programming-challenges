#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<u32, String>> = {
        let mut m = HashMap::new();
        m.insert(0, "foo".to_string());
        m.insert(1, "bar".to_string());
        m.insert(2, "baz".to_string());
        Mutex::new(m)
    };
}

fn main() {
    println!("{:?}", HASHMAP.lock().unwrap().iter());
    HASHMAP.lock().unwrap().remove(&1);
    println!("{:?}", HASHMAP.lock().unwrap().iter());
}