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
    pub group: Vec<StyleSortGroupKey>,
    /// Localization configuration.
    pub localization: Localization,
    /// Sorting configuration.
    pub sort: Vec<StyleSorting>,
    /// Substitution configuration.
    pub substitute: Substitution,
}

impl Default for StyleOptions {
    fn default() -> Self {
        StyleOptions {
            contributors: StyleContributors {
                display_as_sort: None,
                shorten: ShortenListOptions {
                    min: 3,
                    use_first: 3,
                    et_al: AndOptions::Symbol,
                    delimiter_precedes_last: DelimiterLastOptions::Contextual,
                },
                delimiter: DelimiterOptions::Comma,
                and: AndOptions::Symbol,
                label: LabelOptions::Long,
            },
            dates: StyleDate { 
                month: MonthOptions::Long,
            },
            disambiguate: Disambiguation {
                add_names: AddNames::All, // REVIEW: Is this the right default?
                add_year_suffix: true,
            },
            localization: Localization {
                scope: LocalizationScope::Global,
            },
            sort: vec![StyleSorting {
                key: StyleSortGroupKey::Author,
                order: SortOrder::Ascending,
            }],
            group: vec![StyleSortGroupKey::Author],
            substitute: Substitution {
                author: vec![Substitute::Editor, Substitute::Translator, Substitute::Title],
            },
        }
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
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum LocalizationScope {
    Global,
    PerItem,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Disambiguation {
    pub add_names: AddNames,
    pub add_year_suffix: bool,
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

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum AddNames {
    All,
    AllWithInitials,
    ByCite,
    Primary,
    PrimaryWithInitials,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct StyleDate {
    pub month: MonthOptions,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum MonthOptions {
    Long,
    Short,
    Numeric,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum YearSuffixOptions {
    Never,
    Always,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct StyleSorting {
    pub key: StyleSortGroupKey,
    pub order: SortOrder,
}

impl StyleOptions {
    pub fn get_sort_config(&self) -> &[StyleSorting] {
        self.sort.as_slice()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "lowercase", untagged)]
pub enum StyleSortGroupKey {
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
#[serde(rename_all = "lowercase", untagged)]
pub enum DisplayAsSort {
    All,
    First,
    Last,
    None,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "lowercase", untagged)]
pub enum ContributorOptions {
    All,
    First,
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

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum DelimiterLastOptions {
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
#[serde(rename_all = "lowercase")]
pub enum AndOptions {
    Text,
    Symbol,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum LabelOptions {
    Long,
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
    pub contributors: Contributors,
    pub form: ContributorForm,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ContributorForm {
    Long,
    Short,
}
