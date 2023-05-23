pub struct KvStore;

impl KvStore {
    pub fn new() -> KvStore {
        KvStore
    }

    pub fn get(&self, s: String) -> Option<String> {
        None
    }

    pub fn remove(&mut self, s: String) {
    }

    pub fn set(&mut self, k: String, v: String) {
    }
}