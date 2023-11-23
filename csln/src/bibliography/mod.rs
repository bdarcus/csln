use crate::HasFile;
use std::collections::HashMap;
use std::fs;

pub mod reference;
pub use reference::InputReference;

/// A bibliography is a collection of references.
pub type InputBibliography = HashMap<String, InputReference>;

impl HasFile for InputBibliography {
    /// Load and parse a YAML or JSON bibliography file.
    fn from_file(bib_path: &str) -> InputBibliography {
        let contents =
            fs::read_to_string(bib_path).expect("Failed to read bibliography file");
        if bib_path.ends_with(".json") {
            serde_json::from_str(&contents).expect("Failed to parse JSON")
        } else if bib_path.ends_with(".yaml") || bib_path.ends_with(".yml") {
            serde_yaml::from_str(&contents).expect("Failed to parse YAML")
        } else {
            panic!("Style file must be in YAML or JSON format")
        }
    }
}
