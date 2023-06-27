/*
SPDX-License-Identifier: MPL-2.0
SPDX-FileCopyrightText: © 2023 Bruce D'Arcus
*/

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

pub mod locale;
pub mod options;
use options::Config;

pub mod template;
use template::StyleTemplateComponent;

impl Style {
    /// Load and parse a YAML or JSON style file.
    pub fn from_file(style_path: &str) -> Style {
        let contents = fs::read_to_string(style_path).expect("Failed to read style file");
        if style_path.ends_with(".json") {
            serde_json::from_str(&contents).expect("Failed to parse JSON")
        } else if style_path.ends_with(".yaml") || style_path.ends_with(".yml") {
            serde_yaml::from_str(&contents).expect("Failed to parse YAML")
        } else {
            panic!("Style file must be in YAML or JSON format")
        }
    }
}

/// The Style model.
#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema)]
pub struct Style {
    /// Style metadata.
    pub info: StyleInfo,
    pub templates: Option<HashMap<String, StyleTemplate>>,
    /// Parameter groups.
    #[serde(default)]
    pub options: Option<Config>,
    /// The citation specification.
    pub citation: Option<StyleCitation>,
    /// The bibliography specification.
    pub bibliography: Option<StyleBibliography>,
}

/// The Template model.
pub type StyleTemplate = Vec<StyleTemplateComponent>;

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
/// The bibliography specification.
pub struct StyleBibliography {
    pub options: Option<options::Config>,
    pub template: StyleTemplate,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
/// The citation specification.
pub struct StyleCitation {
    pub options: Option<Config>,
    pub template: StyleTemplate,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema)]
/// Style metadata.
pub struct StyleInfo {
    /// The categories the style belongs to; for purposes of indexing.
    pub categories: Option<Vec<StyleCategory>>,
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
pub enum StyleCategory {
    #[serde(rename = "biology")]
    Biology,
    #[serde(rename = "science")]
    Science,
    #[serde(rename = "social science")]
    SocialScience,
}
