use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
pub struct InputReference {
    pub id: Option<String>,
    pub title: Option<String>,
    pub author: Option<Contributor>,
    pub editor: Option<Contributor>,
    pub translator: Option<Contributor>,
    pub issued: Option<String>,
    pub publisher: Option<Contributor>,
    pub url: Option<Url>,
    pub accessed: Option<String>,
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

/// A contributor can be a person or an organzation.
// REVIEW for now, we keep this simple-but-flexible.  We may want to add more structure later.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(untagged)]
pub enum Contributor {
    SimpleName(String),
    StructuredName(StructuredName),
    ContributorList(ContributorList),
}

impl SortContributor for Contributor {
    fn sort_name(&self) -> String {
        match self {
            Contributor::SimpleName(name) => name.to_lowercase(),
            Contributor::StructuredName(contributor) => {
                vec![contributor.family_name.clone(), contributor.given_name.clone()]
                    .join(", ")
                    .to_lowercase()
            }
            Contributor::ContributorList(contributors) => {
                // pass this back to self.sort_name() to create the individual sort names
                contributors.sort_name()
            }
        }
    }
}

impl IntoIterator for ContributorList {
    type Item = Contributor;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl SortContributor for ContributorList {
    fn sort_name(&self) -> String {
        let contributors: Vec<String> = self
            .0
            .iter()
            .map(|c| c.sort_name())
            .collect::<Vec<String>>();
        contributors.join(", ")
    }
}

pub trait DisplayContributor {
    fn display(&self) -> String;
}

pub trait SortContributor {
    fn sort_name(&self) -> String;
}

impl DisplayContributor for Contributor {
    fn display(&self) -> String {
        match self {
            Contributor::SimpleName(name) => name.clone(),
            Contributor::StructuredName(contributor) => {
                vec![contributor.given_name.clone(), contributor.family_name.clone()]
                .join(" ")
            },
            Contributor::ContributorList(contributors) => contributors.display(),
        }
    }
}

impl DisplayContributor for ContributorList {
    fn display(&self) -> String {
        let contributors: Vec<String> = self.0.iter().map(|c| c.display()).collect();
        contributors.join(", ")
    }
}
