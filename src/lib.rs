use std::{
    fs::{File, OpenOptions},
    io::{Write, Read, BufReader, BufRead},
    path::PathBuf, collections::{HashSet, HashMap},
};

use anyhow::Ok;
use error::not_found;
pub use error::{Error, Result};
use serde::{Deserialize, Serialize};

mod error;

pub struct KvStore {
    file: File,
    contents: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
enum LogCmd {
    Set { key: String, value: String },
    Rm { key: String },
}

impl KvStore {
    fn new(f: File) -> Self {
        Self { file: f, contents: HashMap::new() }
    }

    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let mut path_buf: PathBuf = path.into();
        path_buf.push("foo.log");
        let file = OpenOptions::new()
            // XXX is the read(true) needed?
            .read(true)
            .append(true)
            .create(true)
            .open(path_buf)?;
        let reader = BufReader::new(&file);
        let mut contents = HashMap::new();
        for line in reader.lines() {
            let line = line.unwrap();
            let cmd: LogCmd = serde_json::from_str(&line).unwrap();
            match cmd {
                LogCmd::Set { key, value } => {
                    contents.insert(key, value);
                }
                LogCmd::Rm { key } => {
                    contents.remove(&key);
                }
            }
        }
        Ok(KvStore {
            file,
            contents,
        })
    }

    pub fn get(&self, s: String) -> Result<Option<String>> {
        Ok(self.contents.get(&s).cloned())
    }

    pub fn remove(&mut self, s: String) -> Result<()> {
        if self.contents.remove(&s).is_none() {
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
        let cmd = LogCmd::Set { key: k.clone(), value: v.clone() };
        let serialized = serde_json::to_string(&cmd).unwrap();
        writeln!(file, "{serialized}")?;
        self.contents.insert(k, v);
        Ok(())
    }
}
