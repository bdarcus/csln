use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::option::Option;

use bibliography::InputBibliography as Bibliography;
use bibliography::InputReference;
use style::options::{SortOrder, StyleSortGroupKey, StyleSorting};
#[allow(unused_imports)] // for now
use style::template::{
    Contributors, DateForm, Dates, StyleTemplateComponent, StyleTemplateContributor,
};
use style::Style;

/*
This is the processor code.

The basic design is the same as the csl-next typescript implementation:

The processor takes a style, a bibliography, and a locale, and renders the output.

The primary target is a JSON AST, represented by the ProcTemplateComponent struct.
 */

/// Load and parse a YAML or JSON style file.
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

/// Load and parse a YAML or JSON bibliography file.
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
    /// The input style.
    style: Style,
    /// The input bibliography.
    bibliography: Bibliography,
    /// The output locale.
    locale: String,
}

/// The intermediate representation of a StyleTemplateComponent, which is used to render the output.
/// This struct will have two fields: a StyleComponent and a String.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProcTemplateComponent {
    /// The original input style template component, which provides rendering instructions.
    pub template_component: StyleTemplateComponent,
    /// The string to render.
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct ProcHints {
    /// Whether or not the reference needs to be disambiguated.
    pub disamb_condition: bool,
    /// The index of the reference in the group, starting at 1.
    pub group_index: usize,
    /// The number of references in the group.
    pub group_length: usize,
    /// The key of the group.
    pub group_key: String,
}

impl ProcHints {
    pub fn new(
        disamb_condition: bool,
        group_index: usize,
        group_length: usize,
        group_key: String,
    ) -> Self {
        ProcHints {
            disamb_condition,
            group_index,
            group_length,
            group_key,
        }
    }
}

impl Default for ProcHints {
    fn default() -> Self {
        ProcHints {
            disamb_condition: false,
            group_index: 0,
            group_length: 0,
            group_key: "".to_string(),
        }
    }
}

impl Processor {
    /// Get references from the bibliography.
    pub fn get_references(&self) -> Vec<InputReference> {
        let mut references = Vec::new();
        for (key, value) in &self.bibliography {
            let mut reference = value.clone();
            reference.id = Some(key.clone());
            references.push(reference);
        }
        references
    }

    /// Get a reference from the bibliography by id/citekey.
    fn _get_reference(&self, id: &str) -> Option<InputReference> {
        self.bibliography.get(id).cloned()
    }

    /// Sort the references according to instructions in the style.
    pub fn sort_references(&self, references: Vec<InputReference>) -> Vec<InputReference> {
        let mut references = references;
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
                    references.sort_by(|a, b| {
                        // REVIEW would like to review all these unwraps
                        let a_author = a.author.as_ref().unwrap().join(" ").to_lowercase();
                        let b_author = b.author.as_ref().unwrap().join(" ").to_lowercase();
                        if order == "Ascending" {
                            a_author.cmp(&b_author)
                        } else {
                            b_author.cmp(&a_author)
                        }
                    });
                }
                "year" => {
                    references.sort_by(|a, b| {
                        let a_year = a.issued.as_ref().unwrap().parse::<i32>().unwrap();
                        let b_year = b.issued.as_ref().unwrap().parse::<i32>().unwrap();
                        if order == "Ascending" {
                            a_year.cmp(&b_year)
                        } else {
                            b_year.cmp(&a_year)
                        }
                    });
                }
                "title" => {
                    references.sort_by(|a, b| {
                        let a_title = a.title.as_ref().unwrap().to_lowercase();
                        let b_title = b.title.as_ref().unwrap().to_lowercase();
                        if order == "Ascending" {
                            a_title.cmp(&b_title)
                        } else {
                            b_title.cmp(&a_title)
                        }
                    });
                }
                _ => {
                    references.sort_by(|a, b| {
                        let a_author = a.author.as_ref().unwrap().join(" ").to_lowercase();
                        let b_author = b.author.as_ref().unwrap().join(" ").to_lowercase();
                        if order == "Ascending" {
                            a_author.cmp(&b_author)
                        } else {
                            b_author.cmp(&a_author)
                        }
                    });
                }
            }
        }
        references
    }

    // REVIEW strikes me that some of these methods might better be implemented as iterators
    // and also make them asychronous so that they can be run in parallel
    // For a GUI context, that may help make an already fast implementation even faster?
    /// Process the references and return a HashMap of ProcHints.
    pub fn get_proc_hints(&self) -> HashMap<String, ProcHints> {
        let refs = self.get_references();
        let sorted_refs = self.sort_references(refs);
        let grouped_refs = self.group_references(sorted_refs);
        // REVIEW would prefer to avoid using mutable varibles here
        let mut prochs = HashMap::new();
        for (key, group) in grouped_refs {
            let group_len = group.len();
            for (index, reference) in group.into_iter().enumerate() {
                let proch = ProcHints {
                    disamb_condition: false,
                    group_index: index + 1,
                    group_length: group_len,
                    group_key: key.clone(),
                };
                let id = reference.id.as_ref().unwrap().clone();
                prochs.insert(id, proch);
            }
        }
        prochs
    }

    /// Return a string to use for grouping for a given reference, using instructions in the style.
    fn make_group_key(&self, reference: &InputReference) -> String {
        let group_key_config: &[StyleSortGroupKey] = self.style.options.get_group_key_config();
        let group_key = group_key_config
            .iter()
            .map(|key| match key {
                StyleSortGroupKey::Author => "author",
                StyleSortGroupKey::Year => "year",
                StyleSortGroupKey::Title => "title",
            })
            .map(|key| self.string_for_key(reference, key))
            .collect::<Vec<String>>()
            .join(":");
        group_key
    }

    /// Return a string for a given key for a given reference.
    fn string_for_key(&self, reference: &InputReference, key: &str) -> String {
        match key {
            "author" => reference.author.as_ref().unwrap().join(" "),
            "year" => reference.issued.as_ref().unwrap().to_string(),
            "title" => reference.title.as_ref().unwrap().to_string(),
            _ => panic!("Invalid key"),
        }
    }

    // REVIEW not fond of using mutable variables here, but can't figure out Itertools:group_by
    /// Group references according to instructions in the style.
    pub fn group_references(
        &self,
        references: Vec<InputReference>,
    ) -> HashMap<String, Vec<InputReference>> {
        let mut references = references;
        let mut group_map: HashMap<String, Vec<InputReference>> = HashMap::new();
        for reference in references.iter_mut() {
            let group_key = self.make_group_key(reference);
            let group = group_map.entry(group_key).or_insert(Vec::new());
            group.push(reference.clone());
        }
        group_map
    }

    pub fn new(style: Style, bibliography: Bibliography, locale: String) -> Processor {
        Processor {
            style,
            bibliography,
            locale,
        }
    }
}
