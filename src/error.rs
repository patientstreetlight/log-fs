use anyhow::bail;
pub use anyhow::{Error, Result};

pub fn not_found<T>() -> Result<T> {
    bail!("Key not found")
}
