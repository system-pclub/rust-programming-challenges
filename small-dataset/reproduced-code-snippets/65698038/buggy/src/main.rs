#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;
use std::collections::HashMap;

lazy_static! {
    static ref USER_TOKEN_HASHMAP: Mutex<HashMap<&'static str, &'static str>> = {
        let mut m = HashMap::new();
        Mutex::new(m)
    };
}

fn func() {
    let mut _map = USER_TOKEN_HASHMAP.lock().unwrap();
    let user_email = String::from("aaa");
    let user_password = String::from("bbb");
    _map.insert(user_email.as_str(), user_password.as_str());
}

fn main() {}