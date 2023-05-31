use std::fs;
use chrono::NaiveDate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::option::Option;

use bibliography::InputBibliography as Bibliography;
use bibliography::InputReference;
use style::options::{SortOrder, StyleSortGroupKey, StyleSorting};
use style::template::{
    Contributors, DateForm, Dates, StyleTemplateComponent, StyleTemplateContributor,
};
use style::Style;

/*
This is the processor code.

The basic design is the same as the csl-next typescript implementation:

The processor takes a style, a bibliography, and a locale, and renders the output.

The primary target is a JSON AST, represented by the ProcTemplate struct.
 */

pub fn load_style_from_file(style_path: &str) -> Style {
    if style_path.ends_with(".json") {
        load_style_from_json(style_path)
    } else if style_path.ends_with(".yaml") || style_path.ends_with(".yml") {
        load_style_from_yaml(style_path)
    } else {
        panic!("Style file must be in YAML or JSON format")
    }
}

fn load_style_from_yaml(style_path: &str) -> Style {
    let contents = fs::read_to_string(style_path).expect("Failed to read style file");
    serde_yaml::from_str(&contents).expect("Failed to parse YAML")
}

fn load_style_from_json(style_path: &str) -> Style {
    let contents = fs::read_to_string(style_path).expect("Failed to read style file");
    serde_json::from_str(&contents).expect("Failed to parse JSON")
}

pub fn load_bibliography_from_file(bib_path: &str) -> Bibliography {
    if bib_path.ends_with(".json") {
        load_bib_from_json(bib_path)
    } else if bib_path.ends_with(".yaml") || bib_path.ends_with(".yml") {
        load_bib_from_yaml(bib_path)
    } else {
        panic!("Style file must be in YAML or JSON format")
    }
}

fn load_bib_from_yaml(bib_path: &str) -> Bibliography {
    let contents = fs::read_to_string(bib_path).expect("Failed to read style file");
    serde_yaml::from_str(&contents).expect("Failed to parse YAML")
}

fn load_bib_from_json(bib_path: &str) -> Bibliography {
    let contents = fs::read_to_string(bib_path).expect("Failed to read style file");
    serde_json::from_str(&contents).expect("Failed to parse JSON")
}

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
        let sort_config: &[StyleSorting] = self.style.options.get_sort_config();
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
                        let a_year = a.data.issued.as_ref().unwrap().parse::<i32>().unwrap();
                        let b_year = b.data.issued.as_ref().unwrap().parse::<i32>().unwrap();
                        if order == "Ascending" {
                            a_year.cmp(&b_year)
                        } else {
                            b_year.cmp(&a_year)
                        }
                    });
                }
                "title" => {
                    proc_references.sort_by(|a, b| {
                        let a_title = a.data.title.as_ref().unwrap().to_lowercase();
                        let b_title = b.data.title.as_ref().unwrap().to_lowercase();
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
    fn format_names(names: Vec<String>) -> String {
        let mut name_string = String::new();
        if names.len() == 1 {
            name_string = names[0].to_string();
        } else if names.len() == 2 {
            name_string = names.join(" and ");
        } else if names.len() > 2 {
            let last_author = names.last().unwrap();
            let other_authors = &names[..names.len() - 1];
            name_string = other_authors.join(", ");
            name_string.push_str(", and ");
            name_string.push_str(&last_author);
        }
        name_string
    }

    pub fn format_contributors(&self, template_component: StyleTemplateContributor) -> String {
        match template_component.contributor {
            Contributors::Author => {
                let authors = self
                    .data
                    .author
                    .as_ref()
                    .unwrap_or(&Vec::new())
                    .iter()
                    .map(|name| name.to_string())
                    .collect::<Vec<String>>();
                ProcReference::format_names(authors)
            }
            Contributors::Editor => {
                let editors = self
                    .data
                    .editor
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|editor| editor.to_string())
                    .collect::<Vec<String>>();
                ProcReference::format_names(editors)
            }
            Contributors::Translator => {
                let translators = self
                    .data
                    .translator
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|translator| translator.to_string())
                    .collect::<Vec<String>>();
                ProcReference::format_names(translators)
            }
            Contributors::Director => todo!(),
            Contributors::Recipient => todo!(),
            Contributors::Interviewer => todo!(),
            Contributors::Interviewee => todo!(),
            Contributors::Inventor => todo!(),
            Contributors::Counsel => todo!(),
            Contributors::Composer => todo!(),
            Contributors::Publisher => todo!(),
        }
    }

    pub fn format_date(&self, date: Dates, form: DateForm) -> String {
        let date_string: &str = match date {
            Dates::Issued => self.data.issued.as_ref().unwrap(),
            Dates::Accessed => todo!(),
            Dates::OriginalPublished => todo!(),
        };

        let format_string: &str = match form {
            DateForm::Year => "%Y",
            DateForm::YearMonth => "%Y-%m",
            DateForm::Full => "%Y-%m-%d",
            DateForm::MonthDay => "%m-%d",
        };

        // use EDTF instead?
        let date: NaiveDate = NaiveDate::parse_from_str(date_string, "%Y-%m-%d").unwrap();
        let formatted_date: String = date.format(format_string).to_string();
        formatted_date
    }
}
