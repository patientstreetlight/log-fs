use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

use anyhow::Ok;
use error::not_found;
pub use error::{Error, Result};
use serde::{Deserialize, Serialize};

mod error;

pub struct KvStore {
    file: File,
    index: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
enum LogCmd {
    Set { key: String, value: String },
    Rm { key: String },
}

fn open_log_file(dir_path: impl Into<PathBuf>) -> Result<File> {
    let mut path_buf: PathBuf = dir_path.into();
    path_buf.push("foo.log");
    let file = File::options()
        // XXX is the read(true) needed?
        .read(true)
        .append(true)
        .create(true)
        .open(path_buf)?;
    Ok(file)
}

impl KvStore {
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let file = open_log_file(path)?;
        let reader = BufReader::new(&file);
        let mut index = HashMap::new();
        for line in reader.lines() {
            let line = line?;
            let cmd: LogCmd = serde_json::from_str(&line)?;
            match cmd {
                LogCmd::Set { key, value } => {
                    index.insert(key, value);
                }
                LogCmd::Rm { key } => {
                    index.remove(&key);
                }
            }
        }
        Ok(KvStore { file, index })
    }

    pub fn get(&self, s: String) -> Result<Option<String>> {
        Ok(self.index.get(&s).cloned())
    }

    pub fn remove(&mut self, s: String) -> Result<()> {
        if self.index.remove(&s).is_none() {
            return not_found();
        }
        let file = &mut self.file;
        let cmd = LogCmd::Rm { key: s };
        let serialized = serde_json::to_string(&cmd).unwrap();
        writeln!(file, "{serialized}")?;
        Ok(())
    }

    pub fn set(&mut self, k: String, v: String) -> Result<()> {
        let file = &mut self.file;
        let cmd = LogCmd::Set {
            key: k.clone(),
            value: v.clone(),
        };
        let serialized = serde_json::to_string(&cmd).unwrap();
        writeln!(file, "{serialized}")?;
        self.index.insert(k, v);
        Ok(())
    }
}
