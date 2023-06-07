use bibliography::InputBibliography as Bibliography;
use bibliography::InputReference;
use edtf::level_1::Edtf;
use icu::datetime::DateTimeFormatterOptions;
use itertools::Itertools;
use rayon::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::option::Option;
use style::options::StyleOptions;
use style::options::{MonthOptions, SortOrder, StyleSortGroupKey, StyleSorting};
#[allow(unused_imports)] // for now
use style::template::{
    Contributors, DateForm, Dates, StyleTemplateComponent, StyleTemplateContributor,
    StyleTemplateDate, StyleTemplateList, StyleTemplateTitle, Titles,
};
use style::Style;

/*
This is the processor code.

The basic design is the same as the csl-next typescript implementation:

The processor takes a style, a bibliography, and a locale, and renders the output.

The primary target is a JSON AST, represented by the ProcTemplateComponent struct.
 */

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

/// The intermediate representation of a StyleTemplate, which is used to render the output.
pub type ProcTemplate = Vec<ProcTemplateComponent>;

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

pub trait Render<T> {
    fn render(
        &self,
        reference: &InputReference,
        component: &T,
        options: RenderOptions,
    ) -> String;
}

#[allow(dead_code)]
// TODO need to hook this up, but depends on figuring out localized dates first
pub struct RenderOptions<'a> {
    // Options for the style, including default options.
    global: &'a StyleOptions,
    // Options for the citaton or bibliography, that may override the style options.
    local: &'a StyleOptions,
}

// WTD???

pub trait RenderComponent {
    fn render(
        &self,
        reference: &InputReference,
        hints: &ProcHints,
        options: &RenderOptions,
    ) -> String;
}

pub trait RenderDate {
    fn render(
        &self,
        reference: &InputReference,
        hints: &ProcHints,
        options: &RenderOptions,
        // context: &RenderContext<T>,
    ) -> String;

    fn render_date(&self, date_string: &str, format_string: &str) -> String;
}

pub trait RenderTitle {
    fn render(
        &self,
        reference: &InputReference,
        hints: &ProcHints,
        options: &RenderOptions,
        // context: &RenderContext<T>,
    ) -> String;
}

pub trait RenderContributor {
    fn render(
        &self,
        reference: &InputReference,
        hints: &ProcHints,
        options: &RenderOptions,
    ) -> String;
}

impl RenderComponent for StyleTemplateComponent {
    fn render(
        &self,
        reference: &InputReference,
        hints: &ProcHints,
        options: &RenderOptions,
        // context: &RenderContext<T>,
    ) -> String {
        match self {
            StyleTemplateComponent::Title(title) => {
                title.render(reference, hints, options)
            }
            StyleTemplateComponent::Contributor(contributor) => {
                contributor.render(reference, hints, options)
            }
            StyleTemplateComponent::Date(date) => date.render(reference, hints, options),
            StyleTemplateComponent::List(_list) => todo!(),
        }
    }
}

impl<T: RenderContributor + ?Sized> dyn Render<T> {
    pub fn render(names: Vec<String>) -> String {
        names.join(", ")
    }
}

impl RenderTitle for StyleTemplateTitle {
    fn render(
        &self,
        reference: &InputReference,
        _hints: &ProcHints,
        _options: &RenderOptions,
    ) -> String {
        let title: &str = match &self.title {
            Titles::Title => reference.title.as_ref().unwrap(),
            Titles::ContainerTitle => todo!(),
        };
        title.to_string()
    }
}

impl RenderContributor for StyleTemplateContributor {
    fn render(
        &self,
        reference: &InputReference,
        _hints: &ProcHints,
        _options: &RenderOptions,
    ) -> String {
        match &self.contributor {
            Contributors::Author => {
                let authors = reference
                    .author
                    .as_ref()
                    .unwrap_or(&Vec::new())
                    .par_iter()
                    .map(|name| name.to_string())
                    .collect::<Vec<String>>();
                authors.join(", ")
            }
            Contributors::Editor => {
                let editors = reference
                    .editor
                    .as_ref()
                    .unwrap()
                    .par_iter()
                    .map(|editor| editor.to_string())
                    .collect::<Vec<String>>();
                editors.join(", ")
            }
            Contributors::Translator => {
                let translators = reference
                    .translator
                    .as_ref()
                    .unwrap()
                    .par_iter()
                    .map(|translator| translator.to_string())
                    .collect::<Vec<String>>();
                translators.join(", ")
            }
            Contributors::Director => todo!(),
            Contributors::Publisher => todo!(),
            Contributors::Recipient => todo!(),
            Contributors::Interviewer => todo!(),
            Contributors::Interviewee => todo!(),
            Contributors::Composer => todo!(),
            Contributors::Inventor => todo!(),
            Contributors::Counsel => todo!(),
        }
    }
}

impl RenderDate for StyleTemplateDate {
    fn render(
        &self,
        reference: &InputReference,
        hints: &ProcHints,
        _options: &RenderOptions,
    ) -> String {
        let date_string: String = match self.date {
            Dates::Issued => reference.issued.clone().unwrap(),
            Dates::Accessed => reference.accessed.clone().unwrap(),
            Dates::OriginalPublished => todo!(),
        };
        let parsed_date: Edtf = match Edtf::parse(&date_string) {
            Ok(edtf) => edtf,
            Err(_) => return date_string.to_string(),
        };

        let formatted_date: String = match self.form {
            DateForm::Year => parsed_date.as_date().unwrap().year().to_string(),
            DateForm::YearMonth => todo!(),
            DateForm::Full => todo!(),
            DateForm::MonthDay => todo!(),
        };

        // TODO: implement this along with localized dates
        fn _config_fmt(options: &RenderOptions) -> DateTimeFormatterOptions {
            match options.global.dates.month {
                MonthOptions::Long => todo!("long"),
                MonthOptions::Short => todo!("short"),
                MonthOptions::Numeric => todo!("numeric"),
            };
        }

        fn int_to_letter(n: u32) -> String {
            let c = n + 97;
            match char::from_u32(c) {
                Some(ch) => ch.to_string(),
                None => "".to_string(),
            }
        }

        let suffix = if hints.disamb_condition {
            int_to_letter((hints.group_index % 26) as u32)
        } else {
            "".to_string()
        };

        formatted_date + &suffix
    }
    // REVIEW may want to just remove this; not clear ATM
    fn render_date(&self, date_string: &str, _format_string: &str) -> String {
        let edtf_date: Edtf = Edtf::parse(date_string).unwrap();
        let formatted_date: String = match edtf_date {
            // TODO need localized date rendering, using format_string
            Edtf::Date(date) => date.to_string(),
            Edtf::DateTime { .. } => todo!(),
            Edtf::Interval { .. } => todo!(),
            Edtf::IntervalFrom { .. } => todo!(),
            Edtf::IntervalTo { .. } => todo!(),
            Edtf::YYear { .. } => todo!(),
        };

        formatted_date
    }
}

#[test]
fn render_year() {
    let component = StyleTemplateDate {
        date: Dates::Issued,
        form: DateForm::Year,
        rendering: None,
    };
    let reference = InputReference {
        id: Some("test".to_string()),
        issued: Some("2020-01-01".to_string()),
        ..Default::default()
    };
    let options = RenderOptions {
        global: &StyleOptions::default(),
        local: &StyleOptions::default(),
    };
    let rendered_date = component.render(&reference, &ProcHints::default(), &options);
    assert_eq!(rendered_date, "2020");
}

impl Processor {
    /// Render references to AST.
    #[inline]
    pub fn render_references(&self) -> Vec<ProcTemplate> {
        let sorted_references = self.sort_references(self.get_references());
        sorted_references
            .par_iter()
            .map(|reference| self.render_reference(reference))
            .collect()
    }

    /// Render a reference to AST.
    fn render_reference(&self, reference: &InputReference) -> Vec<ProcTemplateComponent> {
        let bibliography_style = self.style.bibliography.clone();
        bibliography_style
            .map(|style| {
                style
                    .template
                    .par_iter()
                    .map(|component| self.render_template_component(component, reference))
                    .collect()
            })
            .unwrap_or_else(std::vec::Vec::new)
    }

    fn get_render_options(&self) -> RenderOptions {
        RenderOptions {
            global: &self.style.options,
            // TTODO: get local options
            local: &self.style.options,
        }
    }

    fn render_template_component(
        &self,
        component: &StyleTemplateComponent,
        reference: &InputReference,
    ) -> ProcTemplateComponent {
        let hints = self.get_proc_hints();
        let reference_id = reference.id.as_ref().unwrap();
        let hint = hints.get(reference_id).cloned().unwrap_or_default();
        let options = self.get_render_options();
        ProcTemplateComponent {
            template_component: component.clone(),
            value: component.render(reference, &hint, &options),
        }
    }

    /// Get references from the bibliography.
    pub fn get_references(&self) -> Vec<InputReference> {
        self.bibliography
            .iter()
            .map(|(key, reference)| {
                let mut input_reference = reference.clone();
                input_reference.id = Some(key.clone());
                input_reference
            })
            .collect()
    }

    /// Get a reference from the bibliography by id/citekey.
    pub fn get_reference(&self, id: &str) -> Option<InputReference> {
        self.bibliography.get(id).cloned()
    }

    /// Sort the references according to instructions in the style.
    #[inline]
    pub fn sort_references(
        &self,
        references: Vec<InputReference>,
    ) -> Vec<InputReference> {
        let mut references = references;
        let sort_config: &[StyleSorting] = self.style.options.get_sort_config();
        sort_config.iter().for_each(|sort| {
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
                    references.par_sort_unstable_by(|a, b| {
                        // REVIEW would like to review all these unwraps
                        let a_author =
                            a.author.as_ref().unwrap().join(" ").to_lowercase();
                        let b_author =
                            b.author.as_ref().unwrap().join(" ").to_lowercase();
                        if order == "Ascending" {
                            a_author.cmp(&b_author)
                        } else {
                            b_author.cmp(&a_author)
                        }
                    });
                }
                "year" => {
                    references.par_sort_unstable_by(|a, b| {
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
                    references.par_sort_unstable_by(|a, b| {
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
                    references.par_sort_unstable_by(|a, b| {
                        let a_author =
                            a.author.as_ref().unwrap().join(" ").to_lowercase();
                        let b_author =
                            b.author.as_ref().unwrap().join(" ").to_lowercase();
                        if order == "Ascending" {
                            a_author.cmp(&b_author)
                        } else {
                            b_author.cmp(&a_author)
                        }
                    });
                }
            }
        });
        references
    }

    /// Process the references and return a HashMap of ProcHints.
    pub fn get_proc_hints(&self) -> HashMap<String, ProcHints> {
        let refs = self.get_references();
        let sorted_refs = self.sort_references(refs);
        let grouped_refs = self.group_references(sorted_refs);
        let proc_hints = grouped_refs
            .iter()
            .flat_map(|(key, group)| {
                let group_len = group.len();
                group.iter().enumerate().map(
                    move |(index, reference)| -> (String, ProcHints) {
                        // TODO will need to generalize.
                        let disambiguate = group_len > 1;
                        let proc_hint = ProcHints {
                            disamb_condition: disambiguate,
                            group_index: index + 1,
                            group_length: group_len,
                            group_key: key.clone(),
                        };
                        (reference.id.as_ref().unwrap().clone(), proc_hint)
                    },
                )
            })
            .collect();
        proc_hints
    }

    /// Return a string to use for grouping for a given reference, using instructions in the style.
    fn make_group_key(&self, reference: &InputReference) -> String {
        let group_key_config: &[StyleSortGroupKey] =
            self.style.options.get_group_key_config();
        let group_key = group_key_config
            // This is likely unnecessary, but just in case.
            .par_iter()
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

    /// Group references according to instructions in the style.
    #[inline]
    pub fn group_references(
        &self,
        references: Vec<InputReference>,
    ) -> HashMap<String, Vec<InputReference>> {
        references
            .into_iter()
            .group_by(|reference| self.make_group_key(reference))
            .into_iter()
            .map(|(key, group)| (key, group.collect()))
            .collect()
    }

    pub fn new(style: Style, bibliography: Bibliography, locale: String) -> Processor {
        Processor { style, bibliography, locale }
    }
}
