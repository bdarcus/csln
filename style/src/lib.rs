use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod options;
use options::StyleOptions;

pub mod template;
use template::StyleTemplateComponent;

/// The CSL-Next style model.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct Style {
    /// Style metadata.
    pub info: StyleInfo,
    pub templates: Option<HashMap<String, StyleTemplate>>,
    /// Parameter groups.
    #[serde(default)]
    pub options: StyleOptions,
    /// The citation specification.
    pub citation: Option<StyleCitation>,
    /// The bibliography specification.
    pub bibliography: Option<StyleBibliography>,
}

pub type StyleTemplate = Vec<StyleTemplateComponent>;

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct StyleBibliography {
    pub options: Option<StyleOptions>,
    pub template: StyleTemplate,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct StyleCitation {
    pub options: Option<StyleOptions>,
    pub template: StyleTemplate,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
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
pub enum StyleCategory {
    #[serde(rename = "biology")]
    Biology,
    #[serde(rename = "science")]
    Science,
    #[serde(rename = "social science")]
    SocialScience,
}
