use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fs;
//use unic_langid::LanguageIdentifier;

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema)]
pub struct Locale {
    pub locale: String,
   // pub options: LocaleOptions,
    pub dates: DateTerms,
    //pub contributors: ContributorTerms,
}

impl Locale {
    pub fn from_file(locale_path: &str) -> Locale {
        let contents = fs::read_to_string(locale_path).expect("Failed to read locale file");
        if locale_path.ends_with(".json") {
            serde_json::from_str(&contents).expect("Failed to parse JSON")
        } else if locale_path.ends_with(".yaml") || locale_path.ends_with(".yml") {
            serde_yaml::from_str(&contents).expect("Failed to parse YAML")
        } else {
            panic!("Locale file must be in YAML or JSON format")
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema)]
pub struct SimpleTerm {
    /// The long form of the term.
    pub long: String,
    /// The short form of the term.
    pub short: String,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema)]
pub struct VerbTerm {
    pub verb: SimpleTerm,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema)]
pub struct ContributorTerms {
    pub author: SimpleTerm,
    pub collection_editor: SimpleTerm,
    pub composer: SimpleTerm,
    pub container_author: SimpleTerm,
    pub director: SimpleTerm,
    pub editor: SimpleTerm,
    pub editorial_director: SimpleTerm,
    pub interviewer: SimpleTerm,
    pub illustrator: SimpleTerm,
    pub original_author: SimpleTerm,
    pub recipient: SimpleTerm,
    pub reviewed_author: SimpleTerm,
    pub translator: SimpleTerm,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct LocaleOptions {
    pub punctuation_in_quotes: bool,
}

/// A struct representing date terms.
///
/// # Fields
///
/// * `month` - vectors containing the full and abbreviated month names.
/// * `seasons` - a map of seasons to their names.
#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema)]
pub struct DateTerms {
    pub months: MonthNames,
    /// The ordered list of seasonal names, starting with Spring.
    /// The list must contain exactly four elements.
    // Note: this corresponds to EDTF level-1; level-2 has many more options.
    #[validate(range(min = 4, max = 4))]
    pub seasons: Vec<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema)]
pub struct MonthNames {
    /// The ordered list of full month names.
    /// The list must contain exactly 12 elements.
    #[validate(range(min = 12, max = 12))]
    pub long: MonthList,
    /// The ordered list of abbreviated month names.
    /// The list must contain exactly 12 elements.
    #[validate(range(min = 12, max = 12))]
    pub short: MonthList,
}

pub type MonthList = Vec<String>;

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum LocalizedTermNameLocator {
    Act,

    Appendix,
    ArticleLocator,

    Book,

    Canon,

    Chapter,

    Column,

    Elocation,

    Equation,

    Figure,

    Folio,

    Line,

    Note,

    Opus,

    Paragraph,

    Rule,

    Scene,

    SubVerbo,

    Table,

    Timestamp,

    TitleLocator,

    Verse,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub enum LocalizedTermNameLocatorNumber {
    Issue,

    Page,

    Part,

    Section,

    Supplement,

    Version,

    Volume,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum LocalizedTermNameMisc {
    Accessed,

    Ad,
    AdvanceOnlinePublication,

    Album,

    And,

    AndOthers,

    Anonymous,

    At,

    AudioRecording,

    AvailableAt,

    Bc,

    Bce,

    By,

    Ce,

    Circa,

    Cited,

    EtAl,

    Film,

    Forthcoming,

    From,

    Henceforth,

    Ibid,

    In,

    InPress,

    Internet,

    Interview,

    Letter,

    LocCit,

    NoDate,

    NoPlace,

    NoPublisher,

    On,

    Online,

    OpCit,

    OriginalWorkPublished,

    PersonalCommunication,

    Podcast,

    PodcastEpisode,

    Preprint,

    PresentedAt,

    RadioBroadcast,

    RadioSeries,

    RadioSeriesEpisode,

    Reference,

    Retrieved,

    ReviewOf,

    Scale,

    SpecialIssue,

    SpecialSection,

    TelevisionBroadcast,

    TelevisionSeries,

    TelevisionSeriesEpisode,

    Video,

    WorkingPaper,
}
