use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct StyleBibliography {
    pub sort: Vec<StyleSorting>,
    pub et_al: EtAlOptions,
    pub delimiter: DelimiterOptions,
    pub and: AndOptions,
    pub prefix: String,
    pub suffix: String,
    pub hanging_indent: bool,
    pub entry_spacing: bool,
    pub second_field_align: bool,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct StyleCitation {
    pub sort: Vec<StyleSorting>,
    pub et_al: EtAlOptions,
    pub delimiter: DelimiterOptions,
    pub and: AndOptions,
    pub prefix: String,
    pub suffix: String,
    pub year_suffix: YearSuffixOptions,
    pub after_punctuation: bool,
    pub entry_spacing: bool,
    pub second_field_align: bool,
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

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct StyleTemplate {
    pub template: String,
    pub macros: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct StyleOptions {
    /// Contributor list formatting configuration.
    pub contributors: Option<StyleContributors>,
    /// Date formatting configuration.
    pub dates: Option<StyleDate>,
    /// Disambiguation configuration of rendererd group display names.
    pub disambiguate: Option<Disambiguation>,
    /// Grouping configuration.
    pub group: Option<Vec<StyleSortGroupKey>>,
    /// Localization configuration.
    pub localization: Option<Localization>,
    /// Sorting configuration.
    pub sort: Option<Vec<StyleSorting>>,
    /// Substitution configuration.
    pub substitute: Option<Substitution>,
}

/// Localization configuration.
///
/// Terms and data localization configuration.
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Localization {
    /// The scope to use for localization.
    ///
    /// "per-item" uses the locale of the reference item, and "global" uses the target language
    /// across all references.
    pub scope: Option<Scope>,
}

/// The scope to use for localization.
///
/// "per-item" uses the locale of the reference item, and "global" uses the target language
/// across all references.
#[derive(Deserialize, Serialize, JsonSchema)]
pub enum Scope {
    #[serde(rename = "global")]
    Global,
    #[serde(rename = "per-item")]
    PerItem,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum LocalizationScope {
    #[serde(rename = "global")]
    Global,
    #[serde(rename = "per-item")]
    PerItem,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Disambiguation {
    #[serde(rename = "addNames")]
    pub add_names: Option<AddNames>,
    #[serde(rename = "addYearSuffix")]
    pub add_year_suffix: Option<bool>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Substitution {
    /// When author is nil, substitute the first non-nil listed variable.
    /// Once a substitution is made, the substituted variable shall be set to nil for purposes of
    /// later rendering.
    pub author: Vec<Substitute>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum Substitute {
    #[serde(rename = "editor")]
    Editor,
    #[serde(rename = "title")]
    Title,
    #[serde(rename = "translator")]
    Translator,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum AddNames {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "all-with-initials")]
    AllWithInitials,
    #[serde(rename = "by-cite")]
    ByCite,
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "primary-with-initials")]
    PrimaryWithInitials,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct StyleDate {
    pub month: MonthOptions,
    pub year_suffix: YearSuffixOptions,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum MonthOptions {
    Long,
    Short,
    Numeric,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum YearSuffixOptions {
    Never,
    Always,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct StyleSorting {
    pub key: StyleSortGroupKey,
    pub direction: SortDirection,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum StyleSortGroupKey {
    Title,
    Author,
    Issued,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum SortDirection {
    Ascending,
    Descending,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct StyleContributors {
    pub names: ContributorOptions,
    pub et_al: EtAlOptions,
    pub delimiter: DelimiterOptions,
    pub and: AndOptions,
    pub label: LabelOptions,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum ContributorOptions {
    All,
    First,
    FirstLast,
    Last,
    LastFirst,
    None,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum EtAlOptions {
    Never,
    Always,
    Min,
    Min2,
    Min3,
    Min4,
    Min5,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum DelimiterOptions {
    Comma,
    SemiColon,
    Period,
    Space,
    Hyphen,
    Ampersand,
    Underscore,
    Colon,
    Hash,
    NoDelimiter,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum AndOptions {
    Text,
    Symbol,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum LabelOptions {
    Long,
    Short,
    Verb,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct StyleTitles {
    pub title: TitleOptions,
    pub subtitle: SubtitleOptions,
    pub short_title: ShortTitleOptions,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum TitleOptions {
    CapitalizeAll,
    CapitalizeFirst,
    Sentence,
    Lowercase,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum SubtitleOptions {
    CapitalizeAll,
    CapitalizeFirst,
    Sentence,
    Lowercase,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum ShortTitleOptions {
    CapitalizeAll,
    CapitalizeFirst,
    Sentence,
    Lowercase,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct StyleTemplateDate {
    pub date: String,
    pub form: DateForm,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum DateForm {
    Text,
    Numeric,
    Roman,
    Ordinal,
    Short,
    Long,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct StyleTemplateTitle {
    pub title: String,
    pub form: TitleForm,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum TitleForm {
    Short,
    Long,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct StyleTemplateContributors {
    pub contributors: String,
    pub form: ContributorForm,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum ContributorForm {
    Long,
    Short,
}
