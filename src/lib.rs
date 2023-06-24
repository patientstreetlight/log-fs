use std::{
    fs::{File, OpenOptions},
    io::Write,
    os::unix::prelude::FileExt,
    path::PathBuf,
};

use anyhow::Ok;
pub use error::{Error, Result};
use serde::{Deserialize, Serialize};

mod error;

pub struct KvStore {
    file: File,
}

#[derive(Serialize, Deserialize, Debug)]
enum LogCmd {
    Set { key: String, value: String },
    Rm { key: String },
}

impl KvStore {
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let mut path_buf: PathBuf = path.into();
        path_buf.push("foo.log");
        let file = OpenOptions::new()
            // XXX is the read(true) needed?
            .read(true)
            .append(true)
            .create(true)
            .open(path_buf)?;
        Ok(KvStore { file })
    }

    pub fn get(&self, s: String) -> Result<Option<String>> {
        unimplemented!()
    }

    pub fn remove(&mut self, s: String) -> Result<()> {
        let file = &mut self.file;
        let cmd = LogCmd::Rm { key: s };
        let serialized = serde_json::to_string(&cmd).unwrap();
        writeln!(file, "{serialized}")?;
        Ok(())
    }

    pub fn set(&mut self, k: String, v: String) -> Result<()> {
        let file = &mut self.file;
        let cmd = LogCmd::Set { key: k, value: v };
        let serialized = serde_json::to_string(&cmd).unwrap();
        writeln!(file, "{serialized}")?;
        Ok(())
    }
}
