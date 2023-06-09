// originally converted from the typescript source with quicktype
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A vector of Citation objects.
#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct CitationList(pub Vec<Citation>);

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct Citation {
    /// Local citation rendering option; aka command or style.
    /// Both are more general than author-date styles, and can apply to any citation style.
    pub mode: CitationModeType,
    /// The string that prefaces a list of citation references.
    pub prefix: Option<String>,
    /// A vector of CitatoinReference objects.
    pub references: Vec<CitationReference>,
    /// A string that follows a list of citation references.
    pub suffix: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum CitationModeType {
    /// Places the author inline in the text; also known as "narrative" or "in text" citations.
    Integral,
    /// Places the author in the citation and/or bibliography or reference entry.
    #[default]
    NonIntegral,
}

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CitationReference {
    /// A string that prefaces the citation reference.
    pub prefix: Option<String>,
    /// The unique identifier token for the citation reference.
    pub ref_id: String,
    /// An array of locator key-values and/or strings.
    pub suffix: Option<Vec<Locator>>,
}

#[allow(clippy::large_enum_variant)] // REVIEW is this a problem?
/// A key-value object, or a string.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum Locator {
    KeyValue(LocatorKeyValue),
    String(String),
}

pub type LocatorKeyValue = (LocatorTerm, String);

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum LocatorTerm {
    Book,
    Chapter,
    Column,
    Figure,
    Folio,
    Line,
    Note,
    Number,
    Opus,
    #[default]
    Page,
    Paragraph,
    Part,
    Section,
    SubVerbo,
    Verse,
    Volume,
}
