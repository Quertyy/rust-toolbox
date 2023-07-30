//! # kvs
use std::collections::HashMap;

/// KvStore struct is used to store key value pairs using a HashMap
/// It has one field, items, which is the HashMap
pub struct KvStore {
    /// The HashMap used to store key value pairs, where the key is a String and the value is a String
    pub items: HashMap<String, String>,
}

/// Implementation of KvStore
impl KvStore {
    /// Creates a new empty KvStore
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    /// Sets the value of a key in the KvStore
    pub fn set(&mut self, key: String, value: String) {
        self.items.insert(key, value);
    }

    /// Gets the value of a key in the KvStore
    pub fn get(&self, key: String) -> Option<String> {
        self.items.get(&key).cloned()
    }

    /// Removes a key from the KvStore
    pub fn remove(&mut self, key: String) {
        self.items.remove(&key);
    }
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}
