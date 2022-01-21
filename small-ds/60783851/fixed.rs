use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Mutex;

pub struct ConcurrentMap<K, V> {
    elems: Mutex<HashMap<K, V>>,
}

impl<K: Hash + Eq, V: Clone> ConcurrentMap<K, V> {
    pub fn new() -> ConcurrentMap<K, V> {
        let map: HashMap<K, V> = HashMap::new();
        ConcurrentMap {
            elems: Mutex::new(map),
        }
    }

    pub fn get(&self, k: &K) -> Option<V> {
        let map = self.elems.lock();
        match map {
            Ok(t) => {
                return match t.get(k) {
                    None => None,
                    Some(v) => Some(v.clone()),
                }
            }
            Err(e) => panic!("GET Concurrent Map Error: {}", e),
        }
    }
}

fn main() {}