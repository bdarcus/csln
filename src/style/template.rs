use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;


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
    Title(StyleTemplateTitle),
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct StyleTemplateContributor {
    pub contributor: Contributors,
    pub form: ContributorForm,
    pub rendering: Option<Rendering>,
    // the string to apply the formatting instructions to
    pub value: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct StyleTemplateDate {
    pub date: String,
    pub form: DateForm,
    pub rendering: Option<Rendering>,
    // the string to apply the formatting instructions to
    pub value: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct StyleTemplateTitle {
    pub title: String,
    pub form: TitleForm,
    pub rendering: Option<Rendering>,
    // the string to apply the formatting instructions to
    pub value: String,
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

/// Traits for rendering the different fields

pub trait Render {
    fn render(&self) -> String;
}

impl fmt::Display for Contributors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Contributors::Author => write!(f, "Author"),
            Contributors::Editor => write!(f, "Editor"),
            Contributors::Translator => write!(f, "Translator"),
            Contributors::Director => write!(f, "Director"),
            Contributors::Recipient => write!(f, "Recipient"),
            Contributors::Interviewer => write!(f, "Interviewer"),
            Contributors::Interviewee => write!(f, "Interviewee"),
            Contributors::Inventor => write!(f, "Inventor"),
            Contributors::Counsel => write!(f, "Counsel"),
            Contributors::Composer => write!(f, "Composer"),
            Contributors::WordsBy => write!(f, "Words by"),
        }
    }
}

// impl Render for StyleTemplateContributor {
//     fn render(&self) -> String {
//         // Render the contributor field based on the form
//         match self.form {
//             ContributorForm::Long => format!("{}: {}", self.contributor, self.as_ref().unwrap().value.as_ref().unwrap()),
//             ContributorForm::Short => self.as_ref().unwrap().value.as_ref().unwrap().clone(),
//         }
//     }
// }

impl Render for StyleTemplateDate {
    fn render(&self) -> String {
        // Render the date field based on the form
        match self.form {
            DateForm::Year => format!("{}: {}", self.date, self.value),
            DateForm::YearMonth => self.value.clone(),
            DateForm::Full => self.value.clone(),
            DateForm::MonthDay => self.value.clone(),
        }
    }
}

impl Render for StyleTemplateTitle {
    fn render(&self) -> String {
        // Render the title field based on the form
        match self.form {
            TitleForm::Long => format!("{}: {}", self.title, self.value),
            TitleForm::Short => self.value.clone(),
        }
    }
}