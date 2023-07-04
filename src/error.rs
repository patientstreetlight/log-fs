use anyhow::bail;
pub use anyhow::{Result, Error};

pub fn not_found<T>() -> Result<T> {
    bail!("Key not found")
}