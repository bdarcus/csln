use std::fmt;

mod template;
use template::{Contributors, DateForm, TitleForm, StyleTemplateDate, StyleTemplateTitle};

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