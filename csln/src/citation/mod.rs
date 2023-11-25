use crate::HasFile;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fs;

impl HasFile for Citations {
    fn from_file(citations_path: &str) -> Citations {
        let contents =
            fs::read_to_string(citations_path).expect("Failed to read citations file");
        if citations_path.ends_with(".json") {
            serde_json::from_str(&contents).expect("Failed to parse JSON")
        } else if citations_path.ends_with(".yaml") || citations_path.ends_with(".yml") {
            serde_yaml::from_str(&contents).expect("Failed to parse YAML")
        } else {
            panic!("Citations file must be in YAML or JSON format")
        }
    }
}

pub type Citations = Vec<Citation>;

/// A vector of Citation objects.
#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct CitationList(pub Vec<Citation>);

/* data Citation a =
  Citation { citationId         :: Maybe Text
           , citationNoteNumber :: Maybe Int
           , citationItems      :: [CitationItem a] }

data CitationItem a =
  CitationItem
  { citationItemId             :: ItemId
  , citationItemLabel          :: Maybe Text
  , citationItemLocator        :: Maybe Text
  , citationItemType           :: CitationItemType
  , citationItemPrefix         :: Maybe a
  , citationItemSuffix         :: Maybe a
  , citationItemData           :: Maybe (Reference a)
  } */

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct Citation {
    pub note_number: Option<i32>,
    pub id: Option<String>,
    /// Local citation rendering option; aka command or style.
    /// These are more general than author-date styles, and can apply to any citation style.
    pub mode: CitationModeType,
    /// The string that prefaces a list of citation references.
    pub prefix: Option<String>,
    /// A vector of CitationItem objects.
    pub citation_items: Vec<CitationItem>,
    /// A string that follows a list of qcitation references.
    pub suffix: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum CitationModeType {
    /// Places the author inline in the text; also known as "narrative" or "in text" citations.
    Integral,
    /// Places the author in the citation and/or bibliography or reference entry.
    #[default]
    NonIntegral,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CitationItem {
    pub label: Option<String>,
    /// A string that prefaces the citation reference.
    pub prefix: Option<String>,
    /// The unique identifier token for the citation reference.
    pub ref_id: String,
    /// An array of locator key-values and/or strings.
    pub suffix: Option<Vec<Locator>>,
}

#[allow(clippy::large_enum_variant)] // REVIEW is this a problem?
/// A key-value object, or a string.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum Locator {
    KeyValue(LocatorKeyValue),
    String(String),
}

pub type LocatorKeyValue = (LocatorTerm, String);

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
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
