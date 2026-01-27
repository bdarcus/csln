use std::collections::HashMap;

/*
SPDX-License-Identifier: MPL-2.0
SPDX-FileCopyrightText: © 2023-2026 Bruce D'Arcus
*/

pub mod reference;
pub use reference::InputReference;

/// A bibliography is a collection of references.
pub type InputBibliography = HashMap<String, InputReference>;

#[cfg(test)]
mod tests {
    use super::reference::Title;
    use super::*;

    #[test]
    fn test_input_bibliography_deserialization() {
        let json = r#"
        {
            "ITEM-1": {
                "id": "ITEM-1",
                "type": "book",
                "title": "Book Title",
                "issued": "2020"
            }
        }
        "#;
        let bib: InputBibliography = serde_json::from_str(json).unwrap();
        assert_eq!(bib.len(), 1);
        // Correct comparison with Title enum
        assert_eq!(
            bib.get("ITEM-1").unwrap().title().as_ref().unwrap(),
            &Title::Single("Book Title".to_string())
        );
    }
}
