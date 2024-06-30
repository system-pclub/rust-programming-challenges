use std::collections::BTreeMap;
use std::sync::{Arc, RwLock, RwLockReadGuard};

pub type Bytes = Vec<u8>;

pub trait IStorageSystem {
    fn open_read<'sys>(&'sys mut self, name: &str) -> Box<dyn IReadOnlyStore<'sys> + 'sys>;
}

pub trait IReadOnlyStore<'sys> {
    fn get(&'sys self, key: &Bytes) -> Option<&'sys Bytes>;
}

type StoreData = BTreeMap<Bytes, Bytes>;

struct InMemoryStorageSystem {
    stores: BTreeMap<String, Arc<RwLock<StoreData>>>,
}

impl InMemoryStorageSystem {
    pub fn new() -> InMemoryStorageSystem {
        InMemoryStorageSystem {
            stores: BTreeMap::new(),
        }
    }
}

impl IStorageSystem for InMemoryStorageSystem {
    fn open_read<'sys>(&'sys mut self, name: &str) -> Box<dyn IReadOnlyStore<'sys> + 'sys> {
        let store_data = self.stores.entry(name.into())
            .or_insert(Arc::new(RwLock::new(StoreData::new())));
        Box::new(ReadOnlyInMemoryStore::new(store_data))
    }
}


struct ReadOnlyInMemoryStore<'sys> {
    data: RwLockReadGuard<'sys, StoreData>,
}


impl<'sys> ReadOnlyInMemoryStore<'sys> {
    fn new(data: &'sys Arc<RwLock<StoreData>>) -> ReadOnlyInMemoryStore<'sys> {
        let locked_data = data.read().unwrap();

        ReadOnlyInMemoryStore {
            data: locked_data,
        }
    }
}


impl<'sys> IReadOnlyStore<'sys> for ReadOnlyInMemoryStore<'sys> {
    fn get(&'sys self, key: &Bytes) -> Option<&'sys Bytes> {
        self.data.get(key)
    }
}
fn main() {
    let mut storage = InMemoryStorageSystem::new();
    let store = storage.open_read("values");

    let key = String::from("dummy").into_bytes();
    let value = store.get(&key);
    println!("{:?}", value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage() {
        let mut storage = InMemoryStorageSystem::new();
        let store = storage.open_read("values");

        let key = String::from("dummy").into_bytes();
        let value = store.get(&key);
        assert_eq!(value, None);
    }
}