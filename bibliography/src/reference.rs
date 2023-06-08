use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;
use style::options::StyleOptions;
use url::Url;
use edtf::level_1::Edtf;

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
pub struct InputReference {
    pub id: Option<String>,
    pub title: Option<String>,
    pub author: Option<Contributor>,
    pub editor: Option<Contributor>,
    pub translator: Option<Contributor>,
    pub issued: Option<RefDate>,
    pub publisher: Option<Contributor>,
    pub url: Option<Url>,
    pub accessed: Option<RefDate>,
    pub note: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
pub struct ContributorList(pub Vec<Contributor>);

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StructuredName {
    pub given_name: String,
    pub family_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(untagged)]
pub enum RefDate {
    Structured(EdtfString),
    Plain(String),
}

pub type EdtfString = String;

impl RefDate {
    pub fn year(&self) -> Option<String> {
        match self {
            RefDate::Structured(date) => {
                let parsed_date: Edtf = match Edtf::parse(&date.to_string()) {
                    Ok(edtf) => edtf,
                    Err(_) => return None,
                };
                Some(parsed_date.as_date().unwrap().year().to_string())
            }
            RefDate::Plain(_) => None,
        }
    }
}

impl fmt::Display for RefDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RefDate::Structured(date) => {
                let parsed_date: Edtf = match Edtf::parse(&date.to_string()) {
                    Ok(edtf) => edtf,
                    Err(_) => return write!(f, "{}", date)
                };
                write!(f, "{}", parsed_date)
            }
            RefDate::Plain(date) => write!(f, "{}", date),
        }
    }
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

impl fmt::Display for Contributor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Contributor::SimpleName(name) => write!(f, "{}", name),
            Contributor::StructuredName(contributor) => {
                write!(f, "{} {}", contributor.given_name, contributor.family_name)
            }
            Contributor::ContributorList(contributors) => {
                write!(f, "{}", contributors)
            }
        }
    }
}

impl fmt::Display for ContributorList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let contributors: Vec<String> =
            self.0.iter().map(|c| c.to_string()).collect::<Vec<String>>();
        write!(f, "{}", contributors.join(", "))
    }
}
pub trait Name {
    fn names(&self, options: StyleOptions, as_sorted: bool) -> String;
}

pub trait NameList {
    /// Return a list of names, formatted according to the given options.
    /// If `as_sorted` is true, the names will be displayed as sorted.
    fn names_list(&self, options: StyleOptions, as_sorted: bool) -> String;
}

impl Name for Contributor {
    // if as_sorted is true, the name will be displayed as sorted.
    fn names(&self, options: StyleOptions, as_sorted: bool) -> String {
        let as_sorted_config = match options.contributors.display_as_sort {
            style::options::DisplayAsSort::All => true,
            style::options::DisplayAsSort::First => true,
            style::options::DisplayAsSort::None => false,
        };
        match self {
            Contributor::SimpleName(name) => name.to_string(),
            Contributor::StructuredName(contributor) => {
                if as_sorted {
                    format!("{}, {}", contributor.family_name, contributor.given_name)
                } else {
                    format!("{} {}", contributor.given_name, contributor.family_name)
                }
            }
            Contributor::ContributorList(contributors) => {
                if as_sorted {
                    let names: Vec<String> = contributors
                        .0
                        .iter()
                        .map(|c| c.names(options.clone(), as_sorted))
                        .collect::<Vec<String>>();
                    names.join(", ")
                } else {
                    contributors.names_list(options, as_sorted_config)
                }
            }
        }
    }
}

impl NameList for ContributorList {
    fn names_list(&self, options: StyleOptions, as_sorted: bool) -> String {
        let names: Vec<String> = self
            .0
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let as_sorted_config = match options.contributors.display_as_sort {
                    style::options::DisplayAsSort::All => true,
                    style::options::DisplayAsSort::First => i == 0,
                    style::options::DisplayAsSort::None => false,
                };
                if as_sorted {
                    c.names(options.clone(), true)
                } else {
                    c.names(options.clone(), as_sorted_config)
                }
            })
            .collect::<Vec<String>>();
        names.join(", ")
    }
}