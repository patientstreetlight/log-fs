use anyhow::bail;
pub use anyhow::{Error, Result};

pub fn not_found<T>() -> Result<T> {
    bail!("Key not found")
}

pub fn invalid_key<T>(expected: &str, actual: &str) -> Result<T> {
    bail!("Expected key {} but found {}", expected, actual)
}

pub fn expected_set_command<T>() -> Result<T> {
    bail!("Expected Set command")
}
