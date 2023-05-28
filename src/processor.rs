use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::option::Option;
use schemars::JsonSchema;

use crate::bibliography::InputBibliography as Bibliography;
use crate::bibliography::InputReference;
use crate::style::StyleTemplateComponent; // FIX
use crate::style::options::{SortOrder, StyleOptions, StyleSorting, StyleSortGroupKey};
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
        let proc_references = self.bibliography
            .values()
            .cloned()
            .map(|input_reference| ProcReference {
                data: input_reference,
                proc_hints: None,
            })
            .collect();

        self.sort_proc_references(proc_references)
    }

    pub fn sort_proc_references (&self, proc_references: Vec<ProcReference>) -> Vec<ProcReference> {
        let mut proc_references = proc_references;
        let sort_config = self.style.options.as_ref().expect("Style options not found").get_sort_config();
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
                },
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
                },
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
                },
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
