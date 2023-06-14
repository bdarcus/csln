use edtf::level_1::Edtf;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;
use style::{locale::MonthList, options::StyleOptions};
use url::Url;
//use icu::calendar::DateTime;

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
pub struct InputReference {
    pub id: Option<String>,
    pub title: Option<String>,
    pub author: Option<Contributor>,
    pub editor: Option<Contributor>,
    pub translator: Option<Contributor>,
    pub issued: Option<EdtfString>,
    pub publisher: Option<Contributor>,
    pub url: Option<Url>,
    pub accessed: Option<EdtfString>,
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
pub struct EdtfString(pub String);

impl EdtfString {
    pub fn as_date(&self) -> Option<Edtf> {
        Edtf::parse(&self.0).ok()
    }

    // TODO do want this or string?
    pub fn year(&self) -> Option<i32> {
        self.as_date().and_then(|d| match d {
            Edtf::Date(date) => Some(date.year()),
            _ => None,
        })
    }

    // FIX these methods need to properly handle missing data
    pub fn month(&self, months: &MonthList) -> Option<String> {
        self.as_date().and_then(|d| match d {
            Edtf::Date(date) => {
                let month = date.month().unwrap();
                let index = month.value().unwrap() as usize - 1;
                if index < months.len() {
                    Some(months[index].clone())
                } else {
                    None
                }
            }
            _ => None,
        })
    }

    pub fn year_month(&self, months: MonthList) -> Option<String> {
        Some(format!("{} {}", self.month(&months)?, self.year().unwrap()))
    }

    pub fn month_day(&self, months: MonthList) -> Option<String> {
        Some(format!("{} {}", self.month(&months)?, self.as_date().unwrap().as_date()?.day().unwrap()))
    }
}

#[test]
fn year_from_edtf_dates() {
    let date = EdtfString("2020-01-01".to_string());
    assert_eq!(date.year(), Some(2020));
    let date = EdtfString("2021-10".to_string());
    assert_eq!(date.year(), Some(2021));
    let date = EdtfString("2022".to_string());
    assert_eq!(date.year(), Some(2022));
}

#[test]
fn month_from_edtf_dates() {
    let months: MonthList = vec![
        "January".to_string(),
        "February".to_string(),
        "March".to_string(),
        "April".to_string(),
        "May".to_string(),
        "June".to_string(),
        "July".to_string(),
        "August".to_string(),
        "September".to_string(),
        "October".to_string(),
        "November".to_string(),
        "December".to_string(),
    ];
    let date = EdtfString("2020-01-01".to_string());
    assert_eq!(date.month(&months), Some("January".to_string()));
    //assert_eq!(date.year_month(months), Some("January 2020".to_string()));
}

impl fmt::Display for EdtfString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: finish this
        let parsed_date: Edtf = match Edtf::parse(&self.0) {
            Ok(edtf) => edtf,
            Err(_) => return write!(f, "{:?}", self),
        };
        write!(f, "{}", parsed_date)
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

#[test]
fn test_display_contributor() {
    let contributor = Contributor::SimpleName("John Smith".to_string());
    assert_eq!(contributor.to_string(), "John Smith");
    let contributor = Contributor::StructuredName(StructuredName {
        given_name: "John".to_string(),
        family_name: "Smith".to_string(),
    });
    assert_eq!(contributor.to_string(), "John Smith");
    let contributor = Contributor::ContributorList(ContributorList(vec![
        Contributor::SimpleName("John Smith".to_string()),
        Contributor::SimpleName("Jane Smith".to_string()),
    ]));
    assert_eq!(contributor.to_string(), "John Smith, Jane Smith");
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

#[test]
fn test_names() {
    let simple = Contributor::SimpleName("John Doe".to_string());
    let structured = Contributor::StructuredName(StructuredName {
        given_name: "John".to_string(),
        family_name: "Doe".to_string(),
    });
    let options = StyleOptions::default();
    assert_eq!(simple.names(options, false), "John Doe");
    let options = StyleOptions::default();
    assert_eq!(
        simple.names(options, true),
        "John Doe",
        "as_sorted=true should not affect a simple name"
    );
    let options = StyleOptions::default();
    assert_eq!(structured.names(options, false), "John Doe");
    let options = StyleOptions::default();
    assert_eq!(structured.names(options, true), "Doe, John");
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

#[test]
fn test_names_list() {
    let contributor_list = ContributorList(vec![
        Contributor::SimpleName("John Doe".to_string()),
        Contributor::SimpleName("Jane Doe".to_string()),
    ]);
    let options = StyleOptions::default();
    assert_eq!(contributor_list.names_list(options, false), "John Doe, Jane Doe");
    let options = StyleOptions::default();
    assert_eq!(
        contributor_list.names_list(options, true),
        "John Doe, Jane Doe",
        "as_sorted=true should not affect simple names"
    );
    let structured_name_list = ContributorList(vec![
        Contributor::StructuredName(StructuredName {
            given_name: "John".to_string(),
            family_name: "Doe".to_string(),
        }),
        Contributor::StructuredName(StructuredName {
            given_name: "Jane".to_string(),
            family_name: "Doe".to_string(),
        }),
    ]);
    let options = StyleOptions::default();
    assert_eq!(structured_name_list.names_list(options, false), "John Doe, Jane Doe");
    let options = StyleOptions::default();
    assert_eq!(structured_name_list.names_list(options, true), "Doe, John, Doe, Jane");
    let options = StyleOptions {
        contributors: style::options::StyleContributors {
            display_as_sort: style::options::DisplayAsSort::First,
            ..style::options::StyleContributors::default()
        },
        ..style::options::StyleOptions::default()
    };
    assert_eq!(structured_name_list.names_list(options, false), "Doe, John, Jane Doe");
}
