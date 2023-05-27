mod error;

use std::{collections::HashMap, path::PathBuf};

pub use error::{Error, Result};

#[derive(Default)]
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    pub fn open(_path: impl Into<PathBuf>) -> Result<KvStore> {
        unimplemented!()
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, s: String) -> Result<Option<String>> {
        Ok(self.store.get(&s).cloned())
    }

    pub fn remove(&mut self, s: String) -> Result<()> {
        self.store.remove(&s);
        Ok(())
    }

    pub fn set(&mut self, k: String, v: String) -> Result<()> {
        self.store.insert(k, v);
        Ok(())
    }
}
