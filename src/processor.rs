use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::option::Option;

use crate::bibliography::InputBibliography as Bibliography;
use crate::bibliography::InputReference;
use crate::style::options::StyleSorting;
use crate::style::Style;

/*
This is the processor code for rendering templates.

The basic design is the same as the csl-next typescript implementation:

The processor takes a style, a bibliography, and a locale, and renders the output.

The primary target is an AST, represented by the ProcRerence struct.
 */

/// The processor struct, which takes a style, a bibliography, and a locale, and renders the output.
#[derive(Debug, Deserialize, Serialize)]
pub struct Processor {
    style: Style,
    bibliography: Bibliography,
    locale: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProcHints {
    procValue: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProcReference {
    pub data: Option<InputReference>,
    pub procHints: Option<ProcHints>,
}

impl Processor {
    fn getProcReferences(&self) -> Vec<ProcReference> {
        // here return a vector of ProcReference structs from the bibliography
        // use iter and map to construct the vector
        // for each reference in the bibliography, construct a ProcReference
        self.bibliography
            .values()
            .cloned()
            .map(|input_reference| ProcReference {
                data: Some(input_reference),
                procHints: None,
            })
            .collect()
    }

    pub fn new(style: Style, bibliography: Bibliography, locale: String) -> Processor {
        Processor {
            style,
            bibliography,
            locale,
        }
    }

    pub fn render(&self) -> String {
        let proc_references = self.getProcReferences();
        let grouped_references = self.groupReferences(proc_references);
        let rendered_references = self.renderReferences(grouped_references);
        let rendered_bibliography = self.renderBibliography(rendered_references);
        rendered_bibliography
    }

    pub fn groupReferences(&self, references: Vec<ProcReference>) -> Vec<ProcReference> {
        let mut grouped_references: Vec<ProcReference> = Vec::new();
        let mut group_index: usize = 0;
        let mut group_length: usize = 0;
        let mut group_values: Vec<String> = Vec::new();

        for reference in references {
            let mut proc_hints = ProcHints {
                procValue: String::new(),
            };

            for group_key in &self.style.group_keys {
                let group_value = match group_key.as_str() {
                    "author" => reference
                        .data
                        .as_ref()
                        .unwrap()
                        .as_ref().unwrap()
                        .map(|v| v.clone())
                        .unwrap_or_default()
                        .clone()
                        .unwrap_or_default()
                        .join(", "),
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
            proc_hints.procValue = format!("{} {}", group_index, proc_hints.procValue);

            let mut grouped_reference = reference.clone();
            grouped_reference.procHints = Some(proc_hints);
            grouped_references.push(grouped_reference);
        }

        grouped_references
    }

    pub fn sortReferences(
        &self,
        references: Vec<ProcReference>,
        sort_objects: Vec<StyleSorting>,
    ) -> Vec<ProcReference> {
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
                        let a_date = a
                            .data
                            .as_ref()
                            .unwrap()
                            .issued
                            .clone()
                            .unwrap_or_default()
                            .date_parts
                            .clone()
                            .unwrap_or_default()
                            .join("-");
                        let b_date = b
                            .data
                            .as_ref()
                            .unwrap()
                            .issued
                            .clone()
                            .unwrap_or_default()
                            .date_parts
                            .clone()
                            .unwrap_or_default()
                            .join("-");
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

    fn renderReferences(&self, references: Vec<ProcReference>) -> Vec<ProcReference> {
        let mut rendered_references: Vec<ProcReference> = Vec::new();
        let mut rendered_reference: ProcReference;
        let mut rendered_reference_data: InputReference;
        let mut rendered_reference_data_value: String;

        for reference in references {
            rendered_reference = reference.clone();
            rendered_reference_data = reference.data.unwrap();
            rendered_reference_data_value = String::new();

            for variable in self.style.variables.clone() {
                let variable_value = match variable.key.as_str() {
                    "author" => rendered_reference_data
                        .author
                        .clone()
                        .unwrap_or_default()
                        .join(", "),
                    "title" => rendered_reference_data.title.clone().unwrap_or_default(),
                    "date" => rendered_reference_data
                        .issued
                        .clone()
                        .unwrap_or_default()
                        .date_parts
                        .clone()
                        .unwrap_or_default()
                        .join("-"),
                    _ => String::new(),
                };

                if rendered_reference_data_value.is_empty() {
                    rendered_reference_data_value = variable_value.clone();
                } else {
                    rendered_reference_data_value =
                        format!("{} {}", rendered_reference_data_value, variable_value);
                }
            }

            rendered_reference.data = Some(rendered_reference_data);
            rendered_references.push(rendered_reference);
        }

        rendered_references
    }

    fn renderReference(&self, reference: ProcReference) -> String {
        let mut rendered_reference: String = String::new();
        let mut rendered_reference_data: InputReference;
        let mut rendered_reference_data_value: String;

        rendered_reference_data = reference.data.unwrap();
        rendered_reference_data_value = String::new();

        for variable in self.style.variables.clone() {
            let variable_value = match variable.key.as_str() {
                "author" => rendered_reference_data
                    .author
                    .clone()
                    .unwrap_or_default()
                    .join(", "),
                "title" => rendered_reference_data.title.clone().unwrap_or_default(),
                "date" => rendered_reference_data
                    .issued
                    .clone()
                    .unwrap_or_default()
                    .date_parts
                    .clone()
                    .unwrap_or_default()
                    .join("-"),
                _ => String::new(),
            };

            if rendered_reference_data_value.is_empty() {
                rendered_reference_data_value = variable_value.clone();
            } else {
                rendered_reference_data_value =
                    format!("{} {}", rendered_reference_data_value, variable_value);
            }
        }

        rendered_reference = rendered_reference_data_value;

        rendered_reference
    }

    fn renderBibliography(&self, references: Vec<ProcReference>) -> String {
        let mut rendered_bibliography: String = String::new();
        let mut rendered_reference: String;

        for reference in references {
            rendered_reference = self.renderReference(reference);
            rendered_bibliography = format!("{}\n{}", rendered_bibliography, rendered_reference);
        }

        rendered_bibliography
    }
}
