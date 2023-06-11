/*
SPDX-License-Identifier: MPL-2.0
SPDX-FileCopyrightText: © 2023 Bruce D'Arcus

This is a small module that defines basic data types and functions for formatting.

Some of the ideas and code are adapted from the `typst-hayagriva` crate.
 */

use bibliography::reference::InputReference;
use edtf::level_1::Edtf;
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;
use style::options::StyleOptions;
use unic_langid::{LanguageIdentifier, LanguageIdentifierError};
use url::Url;
//use std::borrow::Cow;

/*
This section almost-completely adapted from HayaGriva.

The primary differences:

1. We use `serde` for serialization and deserialization.
2. A more general notion of Contributor.
3. Date is a string, not a struct; either EDTF or literal.
4. Use a different model for the `Entry` struct.
5. Use traits for shared functionality.
 */

/// The data types that can possibly be held by the various fields of an
/// [`InputReference`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Value {
    /// A [Title] containing a canonical value and optionally translations and
    /// shorthands, all of which are formattable.
    Title(Title),
    /// A string to be reproduced as-is.
    Text(String),
    /// An integer.
    Integer(i64),
    /// A date string, either EDTF or literal.
    Date(RefDate),
    /// A Contributor.  The model allows single strings, a person names struct, or a list of either.
    Contributor(Contributor),
    /// This could be both an Integer or a Number.
    IntegerOrText(NumOrStr),
    /// A range between two integers.
    Range(std::ops::Range<i64>),
    // /// A duration (of a song or an performance for example).
    // Duration(Duration),
    // /// A part of a period.
    // TimeRange(std::ops::Range<Duration>),
    /// URL.
    Url(Url),
    /// TODO: couldn''t figure out how to do this right.
    Language(LangID),
}

#[derive(Clone, Debug, PartialEq)]
pub struct LangID(LanguageIdentifier);

impl<'de> Deserialize<'de> for LangID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom).map(LangID)
    }
}

impl FromStr for LangID {
    type Err = LanguageIdentifierError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(LangID(s.parse()?))
    }
}

impl Serialize for LangID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.to_string().serialize(serializer)
    }
}

impl JsonSchema for LangID {
    fn schema_name() -> String {
        "LangID".to_owned()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
}

/*
Core structs.
 */

/*
Traits.
 */
pub trait Formattable {
    fn render(&self, referemce: InputReference, style: StyleOptions) -> Option<String>;
}

pub trait SortAndGroupAble {
    fn key(&self) -> String;
}

/*
Core Trait implementations.
 */

impl fmt::Display for LangID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // format the LangID value as a string
        write!(f, "LangID({:?})", self)
    }
}

impl Formattable for (LangID, String) {
    fn render(&self, _referemce: InputReference, _style: StyleOptions) -> Option<String> {
        let (lang_id, title) = self;
        Some(format!("{:?} - {}", lang_id, title))
    }
}

impl Formattable for Title {
    fn render(&self, referemce: InputReference, style: StyleOptions) -> Option<String> {
        match self {
            Title::Single(s) => Some(s.clone()),
            Title::Multi(m) => {
                let mut titles = m
                    .iter()
                    .filter_map(|t| t.render(referemce.clone(), style.clone()))
                    .collect::<Vec<_>>();
                titles.sort();
                Some(titles.join(", "))
            }
            Title::Structured(s) => Some(format!(
                "{} ({})",
                s.main.clone().unwrap(),
                s.sub.clone().unwrap().join(", ")
            )),
            Title::MultiStructured(m) => Some(format!("{} items", m.len())),
            Title::Shorthand(s, t) => Some(format!("{} ({})", s, t)),
        }
    }
}

impl SortAndGroupAble for Title {
    fn key(&self) -> String {
        match self {
            Title::Single(s) => s.to_lowercase(),
            Title::Multi(_m) => {
                todo!("Implement this")
            }
            Title::Structured(_s) => {
                todo!("Implement this")
            }
            Title::MultiStructured(_m) => {
                todo!("Implement this")
            }
            Title::Shorthand(s, t) => {
                format!("{} ({})", s, t)
            }
        }
    }
}

impl Formattable for RefDate {
    fn render(&self, _referemce: InputReference, _style: StyleOptions) -> Option<String> {
        match self {
            RefDate::EdtfString(e) => Some(e.to_string()),
            RefDate::Literal(s) => Some(s.to_string()),
        }
    }
}

impl fmt::Display for RefDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RefDate::EdtfString(date) => {
                let parsed_date: Edtf = match Edtf::parse(&date.to_string()) {
                    Ok(edtf) => edtf,
                    Err(_) => return write!(f, "{}", date)
                };
                write!(f, "{}", parsed_date)
            }
            RefDate::Literal(date) => write!(f, "{}", date),
        }
    }
}

#[test]
fn test_edtf_date_fmt () {
    let date = RefDate::EdtfString("2020-01-01".to_string());
    assert_eq!(date.to_string(), "2020-01-01".to_string());
}

impl SortAndGroupAble for RefDate {
    fn key(&self) -> String {
        match self {
            RefDate::EdtfString(date) => {
                let parsed_date: Edtf = Edtf::parse(&date.to_string())
                    .unwrap_or_else(|_| Edtf::from_str("unknown").unwrap());
                // REVIEW should we only be returning the year?
                parsed_date.as_date().unwrap().year().to_string()
            }
            RefDate::Literal(date) => date.to_string(),
        }
    }
}

impl Formattable for Contributor {
    fn render(&self, referemce: InputReference, style: StyleOptions) -> Option<String> {
        // Clone the reference before moving it into the closure.
        let cloned_reference = referemce;
        match self {
            Contributor::SimpleName(s) => Some(s.to_string()),
            Contributor::StructuredName(n) => {
                Some(format!("{} {}", n.family_name, n.given_name,))
            }
            Contributor::ContributorList(l) => {
                let rendered: Vec<String> =
                    l.0.iter()
                        .filter_map(|c| c.render(cloned_reference.clone(), style.clone()))
                        .collect();
                Some(rendered.join("; "))
            }
        }
    }
}

impl SortAndGroupAble for Contributor {
    fn key(&self) -> String {
        // In progress.
        match self {
            Contributor::SimpleName(s) => s.to_lowercase(),
            Contributor::StructuredName(n) => {
                format!(
                    "{}-{}",
                    n.family_name.to_lowercase(),
                    n.given_name.to_lowercase(),
                )
            }
            Contributor::ContributorList(l) => {
                let names: Vec<String> = l.0.iter().map(|c| c.key()).collect();
                names.join(":")
            }
        }
    }
}

#[test]
fn test_contributor_key() {
    let simple = Contributor::SimpleName("John Doe".to_string());
    assert_eq!(simple.key(), "john doe");

    let structured = Contributor::StructuredName(StructuredName {
        given_name: "John".to_string(),
        family_name: "Doe".to_string(),
    });
    assert_eq!(simple.key(), "john doe");
    assert_eq!(structured.key(), "doe-john");

    let simple_list = Contributor::ContributorList(ContributorList(vec![
        Contributor::SimpleName("John Doe".to_string()),
        Contributor::SimpleName("Jane Doe".to_string()),
    ]));
    let structured_list = Contributor::ContributorList(ContributorList(vec![
        Contributor::StructuredName(StructuredName {
            given_name: "John".to_string(),
            family_name: "Doe".to_string(),
        }),
        Contributor::StructuredName(StructuredName {
            given_name: "Jane".to_string(),
            family_name: "Doe".to_string(),
        }),
    ]));
    assert_eq!(simple_list.key(), "john doe:jane doe");
    assert_eq!(structured_list.key(), "doe-john:doe-jane");
}

/// A contributor can be a person or an organzation.
// REVIEW for now, we keep this simple-but-flexible.  We may want to add more structure later.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(untagged)]
pub enum Contributor {
    SimpleName(String),
    StructuredName(StructuredName),
    ContributorList(ContributorList),
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
pub struct ContributorList(pub Vec<Contributor>);

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StructuredName {
    pub given_name: String,
    pub family_name: String,
}

impl fmt::Display for StructuredName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.given_name, self.family_name)
    }
}

#[test]
fn test_structured_name() {
    let name = StructuredName {
        given_name: "John".to_string(),
        family_name: "Doe".to_string(),
    };
    assert_eq!(
        name.to_string(),
        "John Doe".to_string(),
        "StructuredName should be formatted as 'given_name family_name'");
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(untagged)]
pub enum RefDate {
    EdtfString(EdtfString),
    Literal(String),
}

impl Eq for RefDate {}

impl Ord for RefDate {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (RefDate::EdtfString(a), RefDate::EdtfString(b)) => a.cmp(b),
            (RefDate::Literal(a), RefDate::Literal(b)) => a.cmp(b),
            (RefDate::EdtfString(_), RefDate::Literal(_)) => Ordering::Less,
            (RefDate::Literal(_), RefDate::EdtfString(_)) => Ordering::Greater,
        }
    }
}

#[test]
fn test_date_key () {
    let date = RefDate::EdtfString("2020-01-01".to_string());
    assert_eq!(date.key(), "2020".to_string());
}

impl PartialOrd for RefDate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A collection of formattable strings consisting of a title, a translated
/// title, and a shorthand.
// TODO: borrow from typescript model/ csl 1.1 branch
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Title {
    /// A title in a single language.
    Single(String),
    /// A structured title.
    Structured(StructuredTitle),
    /// A title in multiple languages.
    Multi(Vec<(LangID, String)>),
    /// A title in multiple languages.
    MultiStructured(Vec<(LangID, StructuredTitle)>),
    /// A title with a shorthand.
    Shorthand(String, String),
}

#[test]
fn test_title_key() {
    let title = Title::Single("A Title".to_string());
    assert_eq!(title.key(), "a title".to_string());
    let title = Title::Shorthand("a title".to_string(), "AT".to_string());
    assert_eq!(title.key(), "a title (AT)".to_string());
}

/// Where title parts are meaningful, use this struct; CSLN processors will not parse title strings.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
pub struct StructuredTitle {
    pub full: String,
    pub main: Option<String>,
    pub sub: Option<Vec<String>>,
}

pub type EdtfString = String;

impl RefDate {
    pub fn year(&self) -> Option<String> {
        match self {
            RefDate::EdtfString(date) => {
                let parsed_date: Edtf = match Edtf::parse(&date.to_string()) {
                    Ok(edtf) => edtf,
                    Err(_) => return None,
                };
                Some(parsed_date.as_date().unwrap().year().to_string())
            }
            RefDate::Literal(_) => None,
        }
    }
}

/// A value that could be either a number or a string.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum NumOrStr {
    /// It's a number!
    Number(i64),
    /// It's a string!
    Str(String),
}

impl fmt::Display for NumOrStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Number(i) => write!(f, "{}", i),
            Self::Str(s) => write!(f, "{}", s),
        }
    }
}

impl From<NumOrStr> for String {
    fn from(num: NumOrStr) -> Self {
        num.to_string()
    }
}

impl From<NumOrStr> for i64 {
    fn from(num: NumOrStr) -> Self {
        match num {
            NumOrStr::Number(i) => i,
            NumOrStr::Str(s) => s.parse().unwrap_or(0),
        }
    }
}
