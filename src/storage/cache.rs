use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Cache {
    store: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    capacity: usize,
}

impl Cache {
    pub fn new(capacity: usize) -> Self {
        Cache {
            store: Arc::new(Mutex::new(HashMap::new())),
            capacity,
        }
    }

    pub fn insert(&self, key: &str, data: Vec<u8>) {
        let mut store = self.store.lock().unwrap();
        if store.len() >= self.capacity {
            // Simple eviction strategy (LRU could be implemented here)
            let first_key = store.keys().next().cloned();
            if let Some(key) = first_key {
                store.remove(&key);
            }
        }
        store.insert(key.to_string(), data);
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        let store = self.store.lock().unwrap();
        store.get(key).cloned()
    }

    pub fn delete(&self, key: &str) {
        let mut store = self.store.lock().unwrap();
        store.remove(key);
    }
}
