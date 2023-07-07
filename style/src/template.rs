use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Rendering instructions for a template component.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
pub struct Rendering {
    pub emph: Option<bool>,
    pub quote: Option<bool>,
    pub strong: Option<bool>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub wrap: Option<WrapPunctuation>,
}

/// The punctuation to wrap a template component in.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum WrapPunctuation {
    Parentheses,
    Brackets,
    Braces,
}

/// The Template component model. Each item is for a specific datatype.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(untagged)]
#[non_exhaustive]
pub enum StyleTemplateComponent {
    Contributor(StyleTemplateContributor),
    Date(StyleTemplateDate),
    List(StyleTemplateList),
    Title(StyleTemplateTitle),
    Number(StyleTemplateNumber),
    SimpleString(StyleTemplateSimpleString),
}

/// A simple string component, to render a string variable.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct StyleTemplateSimpleString {
    pub variable: Variables,
    pub rendering: Option<Rendering>,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Variables {
    // TODO: add more variables
    Doi,
    Isbn,
    Issn,
}

/// A number component, to render a number.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct StyleTemplateNumber {
    pub number: Numbers,
    pub form: Option<NumberForm>,
    pub rendering: Option<Rendering>,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Numbers {
    Volume,
    Issue,
    Pages,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum NumberForm {
    #[default]
    Numeric,
    Ordinal,
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
    pub contributor: ContributorRole,
    pub form: ContributorForm,
    pub rendering: Option<Rendering>,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum ContributorForm {
    Long,
    Short,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum ContributorRole {
    Author,
    Editor,
    Translator,
    Director,
    Publisher,
    Recipient,
    Interviewer,
    Interviewee,
    Inventor,
    Counsel,
    Composer,
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
#[non_exhaustive]
pub enum Titles {
    /// The primary title for the cited work.
    Title,
    /// The title of a book or other monograph that the cited work is a part of.
    ParentMonograph,
    /// The titles of a periodical or other serial that the cited work is a part of.
    ParentSerial,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum TitleForm {
    Short,
    Long,
}
