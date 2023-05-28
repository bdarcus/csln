use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Rendering instructions for a template component.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct Rendering {
    emph: Option<bool>,
    strong: Option<bool>,
    prefix: Option<String>,
    suffix: Option<String>,
    wrap: Option<WrapPunctuation>,
}

/// The punctuation to wrap a template component in.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum WrapPunctuation {
    Parentheses,
    Brackets,
    Braces,
}

/// The Template component model. Each item is for a specific datatype.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
#[serde(untagged)]
pub enum StyleTemplateComponent {
    Contributor(StyleTemplateContributor),
    Date(StyleTemplateDate),
    List(StyleTemplateList),
    Title(StyleTemplateTitle),
}

/// To render is a list of more than one item; primarily to enable use of a delimiter to join the items.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct StyleTemplateList {
    pub delimiter: Option<DelimiterPunctuation>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub wrap: Option<WrapPunctuation>,
    pub items: Vec<StyleTemplateComponent>,
}

/// The punctuation to use as a delimiter between items in a list.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum DelimiterPunctuation {
    Comma,
    Semicolon,
    Period,
    Colon,
    Ampersand,
    VerticalLine,
    Slash,
    Hyphen,
    Space,
    None,
}

/// A contributor component, to render a list of contributors.
// TODO incomplete
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct StyleTemplateContributor {
    pub contributor: Contributors,
    pub form: ContributorForm,
    pub rendering: Option<Rendering>,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ContributorForm {
    Long,
    Short,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum Contributors {
    Author,
    Editor,
    Translator,
    Director,
    Recipient,
    Interviewer,
    Interviewee,
    Inventor,
    Counsel,
    Composer,
    WordsBy,
}

/// A date component, to render a date.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct StyleTemplateDate {
    pub date: Dates,
    pub form: DateForm,
    pub rendering: Option<Rendering>,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum Dates {
    Issued,
    Accessed,
    OriginalPublished,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum DateForm {
    Year,
    YearMonth,
    Full,
    MonthDay,
}

/// A title component, to render a title.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct StyleTemplateTitle {
    pub title: Titles,
    pub form: Option<TitleForm>,
    pub rendering: Option<Rendering>,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum Titles {
    Title,
    ContainerTitle,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum TitleForm {
    Short,
    Long,
}

