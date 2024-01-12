pub mod style;
use std::path::Path;

use serde::de::DeserializeOwned;
pub use style::Style;

use std::fs;

pub mod bibliography;
pub use bibliography::InputBibliography;
use style::locale::Locale;

use anyhow::{Context, Result};

pub mod citation;

pub trait Parsable: DeserializeOwned {}
impl Parsable for Style {}
impl Parsable for Locale {}
impl Parsable for InputBibliography {}
impl Parsable for citation::Citations {}

pub fn from_file<T: Parsable, P: AsRef<Path>>(path: P) -> Result<T> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    let value = if path.extension().and_then(|s| s.to_str()) == Some("json") {
        serde_json::from_str(&contents).with_context(|| {
            format!("Failed to parse JSON from file: {}", path.display())
        })?
    } else if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
        serde_yaml::from_str(&contents).with_context(|| {
            format!("Failed to parse YAML from file: {}", path.display())
        })?
    } else {
        return Err(anyhow::anyhow!("Unsupported file extension"));
    };

    Ok(value)
}
