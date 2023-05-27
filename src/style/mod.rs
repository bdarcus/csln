use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod options;
use options::StyleOptions;

mod template;
use template::StyleTemplateComponent;

/// A Style.
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Style {
    /// Style metadata.
    pub info: StyleInfo,
    pub templates: Option<HashMap<String, StyleTemplate>>,
    /// Parameter groups.
    pub options: Option<StyleOptions>,
    /// The citation specification.
    pub citation: Option<StyleCitation>,
    /// The bibliography specification.
    pub bibliography: Option<StyleBibliography>,
}

pub type StyleTemplate = Vec<StyleTemplateComponent>;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct StyleBibliography {
    pub options: Option<StyleOptions>,
    pub template: StyleTemplate,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct StyleCitation {
    pub options: Option<StyleOptions>,
    pub template: StyleTemplate,
}

#[derive(Deserialize, Serialize, JsonSchema)]
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

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum StyleCategory {
    #[serde(rename = "biology")]
    Biology,
    #[serde(rename = "science")]
    Science,
    #[serde(rename = "social science")]
    SocialScience,
}
