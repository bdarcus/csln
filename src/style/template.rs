use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Rendering {
    emph: Option<bool>,
    strong: Option<bool>,
    prefix: Option<String>,
    suffix: Option<String>,
    wrap: Option<WrapPunctuation>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum WrapPunctuation {
    Parentheses,
    Brackets,
    Braces,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum StyleTemplateComponent {
    Contributor(StyleTemplateContributor),
    Date(StyleTemplateDate),
    List(StyleTemplateList),
    Title(StyleTemplateTitle),
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct StyleTemplateList {
    pub delimiter: Option<DelimiterPunctuation>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub wrap: Option<WrapPunctuation>,
    pub items: Vec<StyleTemplateComponent>,
}

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

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct StyleTemplateContributor {
    pub contributor: Contributors,
    pub form: ContributorForm,
    pub rendering: Option<Rendering>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct StyleTemplateDate {
    pub date: String,
    pub form: DateForm,
    pub rendering: Option<Rendering>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct StyleTemplateTitle {
    pub title: String,
    pub form: TitleForm,
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

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum TitleForm {
    Short,
    Long,
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
