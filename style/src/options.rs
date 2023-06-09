/* 
SPDX-License-Identifier: MPL-2.0 
SPDX-FileCopyrightText: © 2023 Bruce D'Arcus

This module defines models and traits for basic data types used in CSLN styles and input data.
*/
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::template::Contributors;

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(default)]
pub struct StyleOptions {
    /// Contributor list formatting configuration.
    pub contributors: StyleContributors,
    /// Date formatting configuration.
    pub dates: StyleDate,
    /// Disambiguation configuration of rendererd group display names.
    pub disambiguate: Disambiguation,
    /// Grouping configuration.
    pub group: Vec<SortGroupKey>,
    /// Localization configuration.
    pub localization: Localization,
    /// Sorting configuration.
    pub sort: Vec<Sort>,
    /// Substitution configuration.
    pub substitute: SubstituteOptions,
}

impl Default for StyleOptions {
    fn default() -> Self {
        Self {
            contributors: StyleContributors::default(),
            dates: StyleDate::default(),
            disambiguate: Disambiguation::default(),
            group: GroupOptions::default().group,
            localization: Localization { scope: LocalizationScope::PerItem },
            sort: vec![
                Sort {
                    key: SortGroupKey::Author,
                    order: SortOrder::Ascending,
                },
                Sort {
                    key: SortGroupKey::Year,
                    order: SortOrder::Ascending,
                }
            ],
            substitute: SubstituteOptions::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct Sort {
    pub key: SortGroupKey,
    pub order: SortOrder,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema)]
pub struct DateOptions {
    pub date: StyleDate,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct SubstituteOptions {
    pub substitute: Vec<Substitute>,
}

pub struct GroupOptions {
    pub group: Vec<SortGroupKey>,
}

impl Default for SubstituteOptions {
    fn default() -> Self {
        Self {
            substitute: vec![
                Substitute::Editor,
                Substitute::Title,
                Substitute::Translator,
            ],
        }
    }
}

impl Default for GroupOptions {
    fn default() -> Self {
        Self {
            group: vec![SortGroupKey::Author, SortGroupKey::Year],
        }
    }
}

impl StyleOptions {
    pub fn get_group_key_config(&self) -> &[SortGroupKey] {
        self.group.as_slice()
    }
    pub fn get_sort_config(&self) -> &[Sort] {
        self.sort.as_slice()
    }
    pub fn get_contributors_config(&self) -> &StyleContributors {
        &self.contributors
    }
    pub fn get_disambiguation_config(&self) -> &Disambiguation {
        &self.disambiguate
    }
    pub fn get_localization_config(&self) -> &Localization {
        &self.localization
    }
    pub fn get_date_config(&self) -> &StyleDate {
        &self.dates
    }
}

/// Localization configuration.
///
/// Terms and data localization configuration.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct Localization {
    /// The scope to use for localization.
    ///
    /// "per-item" uses the locale of the reference item, and "global" uses the target language
    /// across all references.
    pub scope: LocalizationScope,
}

/// The scope to use for localization.
///
/// "per-item" uses the locale of the reference item, and "global" uses the target language
/// across all references.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum LocalizationScope {
    Global,
    PerItem,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Disambiguation {
    pub add_names: bool,
    pub add_year_suffix: bool,
}

impl Default for Disambiguation {
    fn default() -> Self {
        Self { add_names: true, add_year_suffix: false }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct Substitution {
    /// When author is nil, substitute the first non-nil listed variable.
    /// Once a substitution is made, the substituted variable shall be set to nil for purposes of
    /// later rendering.
    pub author: Vec<Substitute>,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum Substitute {
    Editor,
    Title,
    Translator,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct StyleDate {
    pub month: MonthOptions,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MonthOptions {
    #[default]
    Long,
    Short,
    Numeric,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "lowercase", untagged)]
pub enum SortGroupKey {
    Title,
    Author,
    Year,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "lowercase", untagged)]
pub enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct StyleContributors {
    pub display_as_sort: DisplayAsSort,
    pub shorten: ShortenListOptions,
    pub delimiter: DelimiterOptions,
    pub and: AndOptions,
    pub label: LabelOptions,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "lowercase", untagged)]
pub enum DisplayAsSort {
    All,
    First,
    #[default]
    None,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
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

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum DelimiterOptions {
    Comma,
    SemiColon,
    Period,
    #[default]
    Space,
    Hyphen,
    Ampersand,
    Underscore,
    Colon,
    Hash,
    NoDelimiter,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AndOptions {
    #[default] // REVIEW: is this correct?
    Text,
    Symbol,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LabelOptions {
    Long,
    #[default]
    Short,
    Verb,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")] // REVIEW: is this correct?
pub struct StyleTitles {
    pub title: TitleOptions,
    pub subtitle: SubtitleOptions,
    pub short_title: ShortTitleOptions,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum TitleOptions {
    CapitalizeAll,
    CapitalizeFirst,
    Lowercase,
    #[default]
    AsIs,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum SubtitleOptions {
    CapitalizeAll,
    CapitalizeFirst,
    Lowercase,
    #[default]
    AsIs,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum ShortTitleOptions {
    CapitalizeAll,
    CapitalizeFirst,
    Lowercase,
    #[default]
    AsIs,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
pub struct StyleTemplateDate {
    pub date: Option<DateStyle>,
    pub time: Option<TimeStyle>,
    pub month: Option<MonthStyle>,
    pub year: Option<YearStyle>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum YearStyle {
    #[default]
    Numeric,
    TwoDigit,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum MonthStyle {
    Numeric,
    #[default] // REVIEW: is this correct?
    Long,
    Short,
    Narrow,
    TwoDigit,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum TimeStyle {
    Full,
    Short,
    #[default] // REVIEW: is this correct?
    Medium,
    Long,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum DateStyle {
    Full,
    Short,
    #[default] // REVIEW: is this correct?
    Long,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct StyleTemplateTitle {
    pub title: String,
    pub form: TitleForm,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum TitleForm {
    Short,
    #[default] // REVIEW: is this correct?
    Long,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct StyleTemplateContributors {
    pub contributors: Contributors,
    pub form: ContributorForm,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum ContributorForm {
    #[default] // REVIEW: is this correct?
    Long,
    Short,
}
