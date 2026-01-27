/*
SPDX-License-Identifier: MPL-2.0
SPDX-FileCopyrightText: © 2023-2026 Bruce D'Arcus
*/

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod locale;
pub mod options;
use options::Config;

pub mod template;
use template::TemplateComponent;

/// The Style model.
#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema)]
pub struct Style {
    /// Style metadata.
    pub info: Info,
    pub templates: Option<HashMap<String, Template>>,
    /// Parameter groups.
    #[serde(default)]
    pub options: Option<Config>,
    /// The citation specification.
    pub citation: Option<Citation>,
    /// The bibliography specification.
    pub bibliography: Option<Bibliography>,
}

/// The Template model.
pub type Template = Vec<TemplateComponent>;

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
/// The bibliography specification.
pub struct Bibliography {
    pub options: Option<options::Config>,
    pub template: Template,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
/// The citation specification.
pub struct Citation {
    pub options: Option<Config>,
    pub template: Template,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema)]
/// Style metadata.
pub struct Info {
    /// The categories the style belongs to; for purposes of indexing.
    pub categories: Option<Vec<Category>>,
    /// The description of the style.
    pub description: Option<String>,
    /// The machine-readable token that uniquely identifies the style.
    pub id: Option<String>,
    /// The human-readable name of the style.
    pub title: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[non_exhaustive]
/// The categories the style belongs to; for purposes of indexing.
pub enum Category {
    #[serde(rename = "biology")]
    Biology,
    #[serde(rename = "science")]
    Science,
    #[serde(rename = "social science")]
    SocialScience,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_deserialization_minimal() {
        let json = r#"
        {
            "info": {
                "title": "Minimal Style",
                "id": "http://example.com/styles/minimal"
            }
        }
        "#;
        let style: Style = serde_json::from_str(json).unwrap();
        assert_eq!(style.info.title.as_ref().unwrap(), "Minimal Style");
        assert!(style.bibliography.is_none());
        assert!(style.citation.is_none());
    }

    #[test]
    fn test_style_deserialization_complex() {
        let json = r#"
        {
            "info": {
                "title": "Complex Style",
                "id": "http://example.com/styles/complex"
            },
            "bibliography": {
                "template": [
                    {
                        "contributor": "author",
                        "form": "long"
                    },
                    {
                        "date": "issued",
                        "form": "year"
                    }
                ]
            }
        }
        "#;
        let style: Style = serde_json::from_str(json).unwrap();
        assert_eq!(style.info.title.as_ref().unwrap(), "Complex Style");
        let bib = style.bibliography.unwrap();
        assert_eq!(bib.template.len(), 2);
    }
}
