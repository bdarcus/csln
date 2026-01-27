/*
SPDX-License-Identifier: MPL-2.0
SPDX-FileCopyrightText: © 2023-2026 Bruce D'Arcus
*/

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_citation_deserialization() {
        let json = r#"
        {
            "citation_items": [
                {
                    "refId": "ITEM-1"
                }
            ],
            "mode": "integral"
        }
        "#;
        let citation: Citation = serde_json::from_str(json).unwrap();
        assert_eq!(citation.citation_items.len(), 1);
        assert_eq!(citation.citation_items[0].ref_id, "ITEM-1");
        // Check enum matches integral
        match citation.mode {
            CitationModeType::Integral => (),
            _ => panic!("Expected Integral mode"),
        }
    }
}
