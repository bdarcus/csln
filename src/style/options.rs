use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
#[serde(rename_all = "camelCase")]
pub struct Disambiguation {
    pub add_names: Option<AddNames>,
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
    pub display_as_sort: Option<DisplayAsSort>,
    pub shorten: ShortenListOptions,
    pub delimiter: DelimiterOptions,
    pub and: AndOptions,
    pub label: LabelOptions,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum DisplayAsSort {
    All,
    First,
    Last,
    None,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum ContributorOptions {
    All,
    First,
    None,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ShortenListOptions {
    pub min: Option<u32>,
    pub use_first: Option<u32>,
    pub et_al: Option<AndOptions>,
    pub delimiter_precedes_last: Option<DelimiterListOptions>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
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
    pub date: Option<DateStyle>,
    pub time: Option<TimeStyle>,
    pub month: Option<MonthStyle>,
    pub year: Option<YearStyle>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum YearStyle {
    Numeric,
    TwoDigit,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum MonthStyle {
    Numeric,
    Long,
    Short,
    Narrow,
    TwoDigit,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum TimeStyle {
    Full,
    Short,
    Medium,
    Long,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum DateStyle {
    Full,
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
