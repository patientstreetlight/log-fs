use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Seek, SeekFrom, Write},
    path::PathBuf,
};

use anyhow::Ok;
use error::{expected_set_command, invalid_key, not_found};
pub use error::{Error, Result};
use serde::{Deserialize, Serialize};

mod error;

pub struct KvStore {
    file: File,
    index: HashMap<String, u64>,
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
        let mut reader = BufReader::new(&file);
        let mut index = HashMap::new();
        let mut offset = 0;
        let mut line = String::new();
        loop {
            let bytes_read = reader.read_line(&mut line)? as u64;
            if bytes_read == 0 {
                break;
            }
            let cmd: LogCmd = serde_json::from_str(&line)?;
            match cmd {
                LogCmd::Set { key, value } => {
                    index.insert(key, offset);
                }
                LogCmd::Rm { key } => {
                    index.remove(&key);
                }
            }
            offset += bytes_read;
            line.clear();
        }
        Ok(KvStore { file, index })
    }

    pub fn get(&self, s: String) -> Result<Option<String>> {
        let offset = self.index.get(&s).cloned();
        let offset = match offset {
            None => return Ok(None),
            Some(offset) => offset,
        };
        let mut file = &self.file;
        file.seek(SeekFrom::Start(offset))?;
        let mut reader = BufReader::new(file);
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let cmd: LogCmd = serde_json::from_str(&line)?;
        let val = match cmd {
            LogCmd::Set { key, value } => {
                if key != s {
                    return invalid_key(&s, &key);
                }
                value
            }
            _ => return expected_set_command(),
        };
        Ok(Some(val))
    }

    pub fn remove(&mut self, s: String) -> Result<()> {
        if self.index.remove(&s).is_none() {
            return not_found();
        }
        let file = &mut self.file;
        file.seek(SeekFrom::End(0))?;
        let cmd = LogCmd::Rm { key: s };
        let serialized = serde_json::to_string(&cmd).unwrap();
        writeln!(file, "{serialized}")?;
        Ok(())
    }

    pub fn set(&mut self, k: String, v: String) -> Result<()> {
        let file = &mut self.file;
        file.seek(SeekFrom::End(0))?;
        let offset = file.stream_position()?;
        let cmd = LogCmd::Set {
            key: k.clone(),
            value: v,
        };
        let serialized = serde_json::to_string(&cmd).unwrap();
        writeln!(file, "{serialized}")?;
        self.index.insert(k, offset);
        Ok(())
    }
}
