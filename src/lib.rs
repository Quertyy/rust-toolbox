use std::collections::HashMap;

pub struct KvStore {
    pub items: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.items.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.items.get(&key).cloned()
    }

    pub fn remove(&mut self, key: String) {
        self.items.remove(&key);
    }
}