use std::fmt;

mod template;
use template::{Contributors, DateForm, TitleForm, StyleTemplateDate, StyleTemplateTitle};
mod bibliography;
use bibliography::Bibliography;
mod style;
use style::Style;
mod reference;
use reference::InputReference;


/* 
This is the processor code for rendering templates.

The basic design is the same as the csl-next typescript implementation:

The processor takes a style, a bibliography, and a locale, and renders the output.

The primary target is an AST, represented by the ProcRerence struct.
 */



/// The processor struct, which takes a style, a bibliography, and a locale, and renders the output.
#[derive(debug::Debug, Deserialize, Serialize)]
pub struct Processor {
    style: Style,
    bibliography: Bibliography,
    locale: string,
}

pub struct ProcHints {
    procValue: String,
}

pub struct ProcReference {
    pub data: Option<InputReference>,
    pub procHints: Option<ProcHints>,
}

/// Traits for rendering the different template components.

/// Retrun a list of references as ProcReference structs.
#[derive(debug::Debug)]
pub impl getReferences for Processor {
    impl Processor {
        fn getReferences(&self) -> Vec<ProcReference> {
            self.bibliography
                .references
                .iter()
                .map(|reference| ProcReference {
                    data: Some(reference.clone()),
                    procHints: None,
                })
                .collect()
        }
    }
}

#[derive(debug::Debug)]
pub impl groupReferences for Processor {
    //  REVIEW: created by copilot with instructions from me; need to test and review
    fn groupReferences(&self) -> Vec<ProcReference>;
    fn groupReferences(&self, references: Vec<ProcReference>, group_keys: Vec<String>) -> Vec<ProcReference> {
        let mut grouped_references: Vec<ProcReference> = Vec::new();
        let mut group_index: usize = 0;
        let mut group_length: usize = 0;
        let mut group_values: Vec<String> = Vec::new();

        for reference in references {
            let mut proc_hints = ProcHints {
                procValue: String::new(),
            };

            for group_key in &group_keys {
                let group_value = match group_key.as_str() {
                    "author" => reference
                        .data
                        .as_ref()
                        .unwrap()
                        .author
                        .clone()
                        .unwrap_or_default(),
                    "title" => reference
                        .data
                        .as_ref()
                        .unwrap()
                        .title
                        .clone()
                        .unwrap_or_default(),
                    "date" => reference
                        .data
                        .as_ref()
                        .unwrap()
                        .issued
                        .clone()
                        .unwrap_or_default()
                        .date_parts
                        .clone()
                        .unwrap_or_default()
                        .join("-"),
                    _ => String::new(),
                };

                if !group_values.contains(&group_value) {
                    group_index += 1;
                    group_length = 0;
                    group_values.push(group_value.clone());
                }

                if proc_hints.procValue.is_empty() {
                    proc_hints.procValue = group_value.clone();
                } else {
                    proc_hints.procValue = format!("{} {}", proc_hints.procValue, group_value);
                }

                group_length += 1;
            }

            proc_hints.procValue = format!("{} ({})", proc_hints.procValue, group_length);
            proc_hints.procValue = proc_hints.procValue.trim().to_string();

            let mut new_reference = reference.clone();
            new_reference.procHints = Some(proc_hints);
            grouped_references.push(new_reference);
        }

        grouped_references
    }
}

#[derive(debug::Debug)]
pub impl sortReferences for Processor {
    fn sortReferences(&self, references: Vec<ProcReference>, sort_objects: Vec<SortObject>) -> Vec<ProcReference> {
        let mut sorted_references = references.clone();
        for sort_object in sort_objects {
            match sort_object.key.as_str() {
                "author" => {
                    sorted_references.sort_by(|a, b| {
                        let a_author = a.data.as_ref().unwrap().author.clone().unwrap_or_default();
                        let b_author = b.data.as_ref().unwrap().author.clone().unwrap_or_default();
                        if sort_object.direction == "asc" {
                            a_author.cmp(&b_author)
                        } else {
                            b_author.cmp(&a_author)
                        }
                    });
                }
                "title" => {
                    sorted_references.sort_by(|a, b| {
                        let a_title = a.data.as_ref().unwrap().title.clone().unwrap_or_default();
                        let b_title = b.data.as_ref().unwrap().title.clone().unwrap_or_default();
                        if sort_object.direction == "asc" {
                            a_title.cmp(&b_title)
                        } else {
                            b_title.cmp(&a_title)
                        }
                    });
                }
                "date" => {
                    sorted_references.sort_by(|a, b| {
                        let a_date = a.data.as_ref().unwrap().issued.clone().unwrap_or_default().date_parts.clone().unwrap_or_default();
                        let b_date = b.data.as_ref().unwrap().issued.clone().unwrap_or_default().date_parts.clone().unwrap_or_default();
                        if sort_object.direction == "asc" {
                            a_date.cmp(&b_date)
                        } else {
                            b_date.cmp(&a_date)
                        }
                    });
                }
                _ => {}
            }
        }
        sorted_references
    }
}

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