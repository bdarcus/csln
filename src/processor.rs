use chrono::format::strftime::StrftimeItems;
use chrono::DateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::option::Option;

use crate::bibliography::InputBibliography as Bibliography;
use crate::bibliography::InputReference;
use crate::style::options::{SortOrder, StyleOptions, StyleSortGroupKey, StyleSorting};
use crate::style::template::DateForm;
use crate::style::template::{
    Contributors, Dates, StyleTemplateComponent, StyleTemplateContributor, StyleTemplateDate,
};
use crate::style::Style;

/*
This is the processor code.

The basic design is the same as the csl-next typescript implementation:

The processor takes a style, a bibliography, and a locale, and renders the output.

The primary target is a JSON AST, represented by the ProcTemplate struct.
 */

/// The processor struct, which takes a style, a bibliography, and a locale, and renders the output.
#[derive(Debug, Deserialize, Serialize)]
pub struct Processor {
    style: Style,
    bibliography: Bibliography,
    locale: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct ProcHints {
    proc_value: String,
    disamb_condition: Option<bool>,
    group_index: Option<u8>,
    group_length: Option<u8>,
    group_key: Option<String>,
}

// write a ProcTemplate struct that will be used to render the output.
// this struct will be used to render the output.
// the struct will have two fields: a ProcReference and a ProcHints.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct ProcTemplate {
    pub template_component: StyleTemplateComponent,
    pub proc_hints: Option<ProcHints>,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct ProcReference {
    pub data: InputReference,
    pub proc_hints: Option<ProcHints>,
}

impl StyleOptions {
    pub fn get_sort_config(&self) -> &[StyleSorting] {
        self.sort.as_ref().map_or(&[], |s| s.as_slice())
    }
}

impl Processor {
    pub fn get_proc_references(&self) -> Vec<ProcReference> {
        // here return a vector of ProcReference structs from the bibliography
        // use iter and map to construct the vector
        // for each reference in the bibliography, construct a ProcReference
        let proc_references = self
            .bibliography
            .values()
            .cloned()
            .map(|input_reference| ProcReference {
                data: input_reference,
                proc_hints: None,
            })
            .collect();

        self.sort_proc_references(proc_references)
    }

    pub fn sort_proc_references(&self, proc_references: Vec<ProcReference>) -> Vec<ProcReference> {
        let mut proc_references = proc_references;
        let sort_config = self
            .style
            .options
            .as_ref()
            .expect("Style options not found")
            .get_sort_config();
        for sort in sort_config {
            let key = match sort.key {
                StyleSortGroupKey::Author => "author",
                StyleSortGroupKey::Year => "year",
                StyleSortGroupKey::Title => "title",
            };
            let order = match sort.order {
                SortOrder::Ascending => "Ascending",
                SortOrder::Descending => "Descending",
            };
            match key {
                "author" => {
                    proc_references.sort_by(|a, b| {
                        let a_author = a.data.author.as_ref().unwrap().join(" ").to_lowercase();
                        let b_author = b.data.author.as_ref().unwrap().join(" ").to_lowercase();
                        if order == "Ascending" {
                            a_author.cmp(&b_author)
                        } else {
                            b_author.cmp(&a_author)
                        }
                    });
                }
                "year" => {
                    proc_references.sort_by(|a, b| {
                        let a_year = a.data.issued.parse::<i32>().unwrap();
                        let b_year = b.data.issued.parse::<i32>().unwrap();
                        if order == "Ascending" {
                            a_year.cmp(&b_year)
                        } else {
                            b_year.cmp(&a_year)
                        }
                    });
                }
                "title" => {
                    proc_references.sort_by(|a, b| {
                        let a_title = a.data.title.to_lowercase();
                        let b_title = b.data.title.to_lowercase();
                        if order == "Ascending" {
                            a_title.cmp(&b_title)
                        } else {
                            b_title.cmp(&a_title)
                        }
                    });
                }
                _ => {
                    proc_references.sort_by(|a, b| {
                        let a_author = a.data.author.as_ref().unwrap().join(" ").to_lowercase();
                        let b_author = b.data.author.as_ref().unwrap().join(" ").to_lowercase();
                        if order == "Ascending" {
                            a_author.cmp(&b_author)
                        } else {
                            b_author.cmp(&a_author)
                        }
                    });
                }
            }
        }
        proc_references
    }

    pub fn new(style: Style, bibliography: Bibliography, locale: String) -> Processor {
        Processor {
            style,
            bibliography,
            locale,
        }
    }
}

impl ProcReference {
    /// When given a StyleTemplateContributor, format the specifiied contributor using the ProcReference data, and returning a string.
    pub fn format_contributors(&self, template_component: StyleTemplateContributor) -> String {
        // mostly done with istructions to copilot, so need to review and test
        match template_component.contributor {
            Contributors::Author => {
                let mut authors = self
                    .data
                    .author
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|author| author.to_string())
                    .collect::<Vec<String>>();
                let mut author_string = String::new();
                if authors.len() == 1 {
                    author_string = authors[0].to_string();
                } else if authors.len() == 2 {
                    author_string = authors.join(" and ");
                } else if authors.len() > 2 {
                    let last_author = authors.pop().unwrap();
                    author_string = authors.join(", ");
                    author_string.push_str(", and ");
                    author_string.push_str(&last_author);
                }
                author_string
            }
            Contributors::Editor => {
                let mut editors = self
                    .data
                    .editor
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|editor| editor.to_string())
                    .collect::<Vec<String>>();
                let mut editor_string = String::new();
                if editors.len() == 1 {
                    editor_string = editors[0].to_string();
                } else if editors.len() == 2 {
                    editor_string = editors.join(" and ");
                } else if editors.len() > 2 {
                    let last_editor = editors.pop().unwrap();
                    editor_string = editors.join(", ");
                    editor_string.push_str(", and ");
                    editor_string.push_str(&last_editor);
                }
                editor_string
            }
            Contributors::Translator => {
                let mut translators = self
                    .data
                    .translator
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|translator| translator.to_string())
                    .collect::<Vec<String>>();
                let mut translator_string = String::new();
                if translators.len() == 1 {
                    translator_string = translators[0].to_string();
                } else if translators.len() == 2 {
                    translator_string = translators.join(" and ");
                } else if translators.len() > 2 {
                    let last_translator = translators.pop().unwrap();
                    translator_string = translators.join(", ");
                    translator_string.push_str(", and ");
                    translator_string.push_str(&last_translator);
                }
                translator_string
            }
            crate::style::template::Contributors::Director => todo!(),
            crate::style::template::Contributors::Recipient => todo!(),
            crate::style::template::Contributors::Interviewer => todo!(),
            crate::style::template::Contributors::Interviewee => todo!(),
            crate::style::template::Contributors::Inventor => todo!(),
            crate::style::template::Contributors::Counsel => todo!(),
            crate::style::template::Contributors::Composer => todo!(),
            crate::style::template::Contributors::WordsBy => todo!(),
        }
    }

    pub fn format_date(
        &self,
        template_component: StyleTemplateDate,
    ) -> String {
        let date = match template_component.date {
            Dates::Issued => {
                let issued = || self.data.issued.to_owned();
                issued
            }
            Dates::Accessed => todo!(),
            Dates::OriginalPublished => todo!(),
        };

        let format_string = match template_component.form {
            DateForm::Year => "%Y",
            DateForm::YearMonth => "%Y-%m",
            DateForm::Full => "%Y-%m-%d",
            DateForm::MonthDay => "%m-%d",
        };

        let format_items = StrftimeItems::new(format_string);

        let date_time = DateTime::parse_from_rfc3339(&date()).unwrap();

        let formatted_date = date_time.format_with_items(format_items).to_string();

        formatted_date
    }
}
