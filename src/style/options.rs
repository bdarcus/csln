use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
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
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
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
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum Scope {
    Global,
    PerItem,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum LocalizationScope {
    Global,
    PerItem,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct Disambiguation {
    pub add_names: Option<AddNames>,
    pub add_year_suffix: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct Substitution {
    /// When author is nil, substitute the first non-nil listed variable.
    /// Once a substitution is made, the substituted variable shall be set to nil for purposes of
    /// later rendering.
    pub author: Vec<Substitute>,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum Substitute {
    Editor,
    Title,
    Translator,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
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

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct StyleDate {
    pub month: MonthOptions,
    pub year_suffix: YearSuffixOptions,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub enum MonthOptions {
    Long,
    Short,
    Numeric,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub enum YearSuffixOptions {
    Never,
    Always,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct StyleSorting {
    pub key: StyleSortGroupKey,
    pub direction: SortDirection,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub enum StyleSortGroupKey {
    Title,
    Author,
    Issued,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct StyleContributors {
    pub display_as_sort: Option<DisplayAsSort>,
    pub shorten: ShortenListOptions,
    pub delimiter: DelimiterOptions,
    pub and: AndOptions,
    pub label: LabelOptions,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum DisplayAsSort {
    All,
    First,
    Last,
    None,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ContributorOptions {
    All,
    First,
    None,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct ShortenListOptions {
    pub min: Option<u8>,
    pub use_first: Option<u8>,
    pub et_al: Option<AndOptions>,
    pub delimiter_precedes_last: Option<DelimiterListOptions>,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum DelimiterListOptions {
    /// Delimiter is only used if preceding name is inverted as a result of the`asSort` parameter. E.g. with `asSort` set to “first”.
    AfterInvertedName,
    /// Delimiter is always used when more than two, regardless of shortening.
    Always,
    /// Delimiter is never used.
    Never,
    /// The delimiter is only used when shortening is applied.
    Contextual,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
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

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum AndOptions {
    Text,
    Symbol,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum LabelOptions {
    Long,
    Short,
    Verb,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct StyleTitles {
    pub title: TitleOptions,
    pub subtitle: SubtitleOptions,
    pub short_title: ShortTitleOptions,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum TitleOptions {
    CapitalizeAll,
    CapitalizeFirst,
    Sentence,
    Lowercase,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum SubtitleOptions {
    CapitalizeAll,
    CapitalizeFirst,
    Sentence,
    Lowercase,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ShortTitleOptions {
    CapitalizeAll,
    CapitalizeFirst,
    Sentence,
    Lowercase,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct StyleTemplateDate {
    pub date: Option<DateStyle>,
    pub time: Option<TimeStyle>,
    pub month: Option<MonthStyle>,
    pub year: Option<YearStyle>,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum YearStyle {
    Numeric,
    TwoDigit,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum MonthStyle {
    Numeric,
    Long,
    Short,
    Narrow,
    TwoDigit,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum TimeStyle {
    Full,
    Short,
    Medium,
    Long,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum DateStyle {
    Full,
    Short,
    Long,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct StyleTemplateTitle {
    pub title: String,
    pub form: TitleForm,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum TitleForm {
    Short,
    Long,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct StyleTemplateContributors {
    pub contributors: String,
    pub form: ContributorForm,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ContributorForm {
    Long,
    Short,
}
