use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Rendering instructions for a template component.
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Rendering {
    emph: Option<bool>,
    strong: Option<bool>,
    prefix: Option<String>,
    suffix: Option<String>,
    wrap: Option<WrapPunctuation>,
}

/// The punctuation to wrap a template component in.
#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum WrapPunctuation {
    Parentheses,
    Brackets,
    Braces,
}

/// The Tepmlate component model. Each component is for a specific datatype.
#[derive(Deserialize, Serialize, JsonSchema)]
pub enum StyleTemplateComponent {
    Contributor(StyleTemplateContributor),
    Date(StyleTemplateDate),
    List(StyleTemplateList),
    Title(StyleTemplateTitle),
}

/// To render is a list of more than one item; primarily to enable use of a delimiter to join the items.
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct StyleTemplateList {
    pub delimiter: Option<DelimiterPunctuation>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub wrap: Option<WrapPunctuation>,
    pub items: Vec<StyleTemplateComponent>,
}

/// The punctuation to use as a delimiter between items in a list.
#[derive(Deserialize, Serialize, JsonSchema)]
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
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct StyleTemplateContributor {
    pub contributor: Contributors,
    pub form: ContributorForm,
    pub rendering: Option<Rendering>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ContributorForm {
    Long,
    Short,
}

#[derive(Deserialize, Serialize, JsonSchema)]
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
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct StyleTemplateDate {
    pub date: String,
    pub form: DateForm,
    pub rendering: Option<Rendering>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum Dates {
    Issued,
    Accessed,
    OriginalPublished,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum DateForm {
    Year,
    YearMonth,
    Full,
    MonthDay,
}

/// A title component, to render a title.
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct StyleTemplateTitle {
    pub title: String,
    pub form: TitleForm,
    pub rendering: Option<Rendering>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum TitleForm {
    Short,
    Long,
}

