use chrono::NaiveDate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
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
    let contents = fs::read_to_string(style_path).expect("Failed to read style file");
    if style_path.ends_with(".json") {
        serde_json::from_str(&contents).expect("Failed to parse JSON")
    } else if style_path.ends_with(".yaml") || style_path.ends_with(".yml") {
        serde_yaml::from_str(&contents).expect("Failed to parse YAML")
    } else {
        panic!("Style file must be in YAML or JSON format")
    }
}

pub fn load_bibliography_from_file(bib_path: &str) -> Bibliography {
    let contents = fs::read_to_string(bib_path).expect("Failed to read bibliography file");
    if bib_path.ends_with(".json") {
        serde_json::from_str(&contents).expect("Failed to parse JSON")
    } else if bib_path.ends_with(".yaml") || bib_path.ends_with(".yml") {
        serde_yaml::from_str(&contents).expect("Failed to parse YAML")
    } else {
        panic!("Style file must be in YAML or JSON format")
    }
}

/// The processor struct, which takes a style, a bibliography, and a locale, and renders the output.
#[derive(Debug, Deserialize, Serialize)]
pub struct Processor {
    style: Style,
    bibliography: Bibliography,
    locale: String,
}

/// The intermedia representation of a StyleTemplateComponent, which is used to render the output.
/// This struct will have two fields: a StyleComponent and a ProcHints.
/// The ProcHints field will be used to store information about the reference that is used to render the output.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProcTemplateComponent {
    template_component: StyleTemplateComponent,
    proc_hints: Option<ProcHints>,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct ProcHints {
    disamb_condition: bool,
    group_index: u8,
    group_length: usize,
    group_key: String,
}

impl Default for ProcHints {
    fn default() -> Self {
        ProcHints {
            disamb_condition: false, // TODO need to dynamically set this
            group_index: 1,
            group_length: 1,
            group_key: "".to_string(), // TODO fix this
        }
    }
}

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
    pub proc_hints: ProcHints,
}

impl Processor {
    pub fn get_all_proc_references(&self) -> Vec<ProcReference> {
        let prefs = self
            .bibliography
            .values()
            .cloned()
            .map(|input_reference| ProcReference {
                data: input_reference,
                proc_hints: ProcHints {
                    disamb_condition: false,
                    group_index: 0,
                    group_length: 0,
                    group_key: "".to_string(),
                },
            })
            .collect();

        let sorted_prefs: Vec<ProcReference> = self.sort_proc_references(prefs);
        let grouped_prefs: HashMap<String, Vec<ProcReference>> =
            self.group_proc_references(sorted_prefs);
        let mut proc_references: Vec<ProcReference> = Vec::new();
        grouped_prefs.iter().for_each(|(group_key, group)| {
            let group_len = group.len();
            group.iter().enumerate().for_each(|(group_index, proc_reference)| {
                let phints = ProcHints {
                    disamb_condition: group_len > 1,
                    group_index: (group_index + 1) as u8,
                    group_length: group_len,
                    group_key: group_key.to_string(),
                };
                let proc_reference = ProcReference {
                    data: proc_reference.data.clone(),
                    proc_hints: phints,
                };
                proc_references.push(proc_reference);
            });
        });
        proc_references
    }

    pub fn get_proc_references(&self) -> Vec<ProcReference> {
        let all_proc_references = self.get_all_proc_references();
        // additional processing goes here
        all_proc_references
    }

    fn make_group_key(&self, proc_reference: &ProcReference) -> String {
        let group_key_config: &[StyleSortGroupKey] = self.style.options.get_group_key_config();
        let group_key = group_key_config
            .iter()
            .map(|key| match key {
                StyleSortGroupKey::Author => "author",
                StyleSortGroupKey::Year => "year",
                StyleSortGroupKey::Title => "title",
            })
            .map(|key| self.string_for_key(proc_reference, key))
            .collect::<Vec<String>>()
            .join(":");
        group_key
    }

    fn string_for_key(&self, proc_reference: &ProcReference, key: &str) -> String {
        match key {
            "author" => proc_reference.data.author.as_ref().unwrap().join(" "),
            "year" => proc_reference.data.issued.as_ref().unwrap().to_string(),
            "title" => proc_reference.data.title.as_ref().unwrap().to_string(),
            _ => panic!("Invalid key"),
        }
    }

    // REVIEW not fond of using mutable variables here, but can't figure out Itertools:group_by
    pub fn group_proc_references(
        &self,
        proc_references: Vec<ProcReference>,
    ) -> HashMap<String, Vec<ProcReference>> {
        let mut proc_references = proc_references;
        let mut group_map: HashMap<String, Vec<ProcReference>> = HashMap::new();
        for proc_reference in proc_references.iter_mut() {
            let group_key = self.make_group_key(proc_reference);
            let group = group_map.entry(group_key).or_insert(Vec::new());
            group.push(proc_reference.clone());
        }
        group_map
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
            name_string.push_str(last_author);
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
