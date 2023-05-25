use std::collections::HashMap;

#[derive(Default)]
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        Self::default()
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
