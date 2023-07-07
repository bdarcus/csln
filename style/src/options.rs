/*
SPDX-License-Identifier: MPL-2.0
SPDX-FileCopyrightText: © 2023 Bruce D'Arcus
*/

//! This submodule defines the configuration groups and options available in CSLN styles.
//!
//! The details are adapted from:
//!
//! 1. The [CSL 1.0 specification][CSL-spec] [options][CSL-options], and its template language (aka [layout][CSL-templates] and [rendering elements][CSL-render]), most notably from names, dates, and other formatting.
//! 2. Patterns observed in the [CSL 1.0 styles repository][CSL-styles].
//! 3. The [BibLaTeX preamble][BLTX] options.
//!
//! In this model, much more logic is configured in these options, and the `template` submodule is comparatively simple.
//! The intent is to make it easier to write and maintain styles, as well as softtware that uses them.
//!
//! ## Style Options
//!
//! The [`Config`] struct defines the configuration groups and options available in CSLN styles.
//!
//! ## Status
//!
//! Still early, with more work needed on adding options, and testing.
//!
//! [CSL-spec]: https://docs.citationstyles.org/en/stable/specification.html
//! [CSL-styles]: https://github.com/citation-style-language/styles
//! [CSL-macros]: https://docs.citationstyles.org/en/stable/specification.html#macros
//! [CSL-templates]: https://docs.citationstyles.org/en/stable/specification.html#layout-1
//! [CSL-render]: https://docs.citationstyles.org/en/stable/specification.html#rendering-elements
//! [CSL-options]: https://docs.citationstyles.org/en/stable/specification.html#options
//! [BLTX]: https://github.com/plk/biblatex
//!

use crate::template::Rendering;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(JsonSchema, Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Config {
    pub substitute: Option<Substitute>,
    pub processing: Option<Processing>,
    pub localize: Option<Localize>,
    pub contributors: Option<ContributorConfig>,
    pub dates: Option<Date>,
    pub titles: Option<Titles>,
}

#[derive(JsonSchema, Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Titles {
    component: Option<Rendering>,
    monograph: Option<Rendering>,
    default: Option<Rendering>,
}

#[derive(JsonSchema, Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum Processing {
    #[default]
    // FIX again, this pattern doesn't work
    AuthorDate,
    Numeric,
    Custom(ProcessingCustom),
}

#[derive(JsonSchema, Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProcessingCustom {
    pub sort: Option<Sort>,
    pub group: Option<Group>,
    pub disambiguate: Option<Disambiguation>,
}

impl Processing {
    pub fn config(&self) -> ProcessingCustom {
        match self {
            Processing::AuthorDate => ProcessingCustom {
                sort: Some(Sort {
                    shorten_names: false,
                    render_substitutions: false,
                    template: vec![
                        SortSpec { key: SortKey::Author, ascending: true },
                        SortSpec { key: SortKey::Year, ascending: true },
                    ],
                }),
                group: Some(Group { template: vec![SortKey::Author, SortKey::Year] }),
                disambiguate: Some(Disambiguation { names: true, year_suffix: true }),
            },
            Processing::Numeric => {
                ProcessingCustom { sort: None, group: None, disambiguate: None }
            }
            Processing::Custom(custom) => custom.clone(),
        }
    }
}

#[test]
fn author_date_config() {
    let config = Processing::AuthorDate.config();
    let sort = config.sort.unwrap_or_default();
    assert_eq!(sort.template[0].key, SortKey::Author);
    assert_eq!(sort.template[1].key, SortKey::Year);
    assert!(config.disambiguate.unwrap().year_suffix);
}

#[derive(JsonSchema, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Disambiguation {
    pub names: bool,
    pub year_suffix: bool,
}

impl Default for Disambiguation {
    fn default() -> Self {
        Self { names: true, year_suffix: false }
    }
}

#[derive(JsonSchema, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Date {
    pub month: MonthFormat,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MonthFormat {
    #[default]
    Long,
    Short,
    Numeric,
}

impl Default for Date {
    fn default() -> Self {
        Self { month: MonthFormat::Long }
    }
}

#[test]
fn date_default_config() {
    let config = Config::default();
    assert_eq!(config.dates.unwrap_or_default().month, MonthFormat::Long);
}

#[derive(JsonSchema, Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContributorConfig {
    /// When to display a contributor's name in sort order.
    pub display_as_sort: Option<DisplayAsSort>,
    /// Shorten the list of contributors.
    pub shorten: Option<ShortenListOptions>,
    /// The delimiter or separator to use between contributors.
    pub delimiter: Option<String>,
    /// Whether to separate the last two contributors with a natural language conjunction, and if so what form it should take.
    pub and: Option<AndOptions>,
    /// When and how to display contributor roles.
    pub role: Option<RoleOptions>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DisplayAsSort {
    All,
    First,
    #[default]
    None,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum AndOptions {
    #[default] // REVIEW: is this correct?
    Text,
    Symbol,
    None,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RoleOptions {
    /// Contributor roles for which to omit the role description.
    ///
    /// The default value is `["author"]`, which omits the role for authors, including for any
    /// author substitutions.
    // TODO
    pub omit: Vec<String>,
    pub form: String, // TODO
    pub rendering: Option<Rendering>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum DelimiterLastOptions {
    /// Delimiter is only used if preceding name is inverted as a result of the`asSort` parameter. E.g. with `asSort` set to “first”.
    AfterInvertedName,
    /// Delimiter is always used when more than two, regardless of shortening.
    Always,
    /// Delimiter is never used.
    Never,
    #[default]
    /// The delimiter is only used when shortening is applied.
    Contextual,
}

#[derive(JsonSchema, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortenListOptions {
    pub min: u8,
    pub use_first: u8,
    pub et_al: AndOptions,
    pub delimiter_precedes_last: DelimiterLastOptions,
}

impl Default for ShortenListOptions {
    // REVIEW these defaults
    fn default() -> Self {
        Self {
            min: 3,
            use_first: 1,
            et_al: AndOptions::default(),
            delimiter_precedes_last: DelimiterLastOptions::default(),
        }
    }
}

#[derive(JsonSchema, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Localize {
    pub scope: Scope,
}

#[derive(JsonSchema, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Scope {
    Global,
    PerItem,
}

impl Default for Localize {
    fn default() -> Self {
        Self { scope: Scope::Global }
    }
}

#[test]
fn localize_config_default() {
    let config = Config::default();
    assert_eq!(config.localize.unwrap_or_default().scope, Scope::Global);
}

#[derive(JsonSchema, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Group {
    pub template: Vec<SortKey>,
}

#[derive(JsonSchema, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Substitute {
    pub template: Vec<SubstituteKey>,
}

impl Default for Substitute {
    fn default() -> Self {
        Self {
            template: vec![
                SubstituteKey::Editor,
                SubstituteKey::Title,
                SubstituteKey::Translator,
            ],
        }
    }
}

#[test]
fn substitute_default() {
    let config = Config::default();
    assert_eq!(config.substitute.unwrap_or_default().template.len(), 3);
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Sort {
    /// Shorten name lists for sorting the same as for display.
    // REVIEW: may need more options here.
    #[serde(default = "default_shorten_names")]
    pub shorten_names: bool,
    /// Use same substitutions for sorting as for rendering.
    #[serde(default = "default_render_substitutions")]
    pub render_substitutions: bool,
    pub template: Vec<SortSpec>,
}

fn default_shorten_names() -> bool {
    false
}

fn default_render_substitutions() -> bool {
    false
}
#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
pub struct SortSpec {
    pub key: SortKey,
    #[serde(default = "default_ascending")]
    pub ascending: bool,
}

fn default_ascending() -> bool {
    true
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum SortKey {
    #[default]
    Author,
    Year,
    Title,
}

#[derive(JsonSchema, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SubstituteKey {
    Editor,
    Title,
    Translator,
}
