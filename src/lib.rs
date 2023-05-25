use std::collections::HashMap;

pub struct KvStore {
    store: HashMap<String, String>,
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            store: HashMap::new()
        }
    }

    pub fn get(&self, s: String) -> Option<String> {
        self.store.get(&s).cloned()
    }

    pub fn remove(&mut self, s: String) {
        self.store.remove(&s);
    }

    pub fn set(&mut self, k: String, v: String) {
        self.store.insert(k, v);
    }
}