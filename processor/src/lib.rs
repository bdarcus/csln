/*
SPDX-License-Identifier: MPL-2.0
SPDX-FileCopyrightText: © 2023 Bruce D'Arcus
*/

use csln::bibliography::reference::InputReference;
use csln::bibliography::reference::{EdtfString, Name, RefID};
use csln::bibliography::InputBibliography as Bibliography;
use csln::citation::Citation;
use csln::style::locale::Locale;
use csln::style::options::{Config, MonthFormat, SortKey, SubstituteKey};
use csln::style::template::{
    ContributorRole, DateForm, Dates, Numbers, TemplateComponent, TemplateContributor,
    TemplateDate, TemplateNumber, TemplateSimpleString, TemplateTitle, Titles, Variables,
    WrapPunctuation,
};
use csln::style::Style;
use icu::datetime::DateTimeFormatterOptions;
use itertools::Itertools;
use rayon::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{self, Debug, Display, Formatter};
use std::option::Option;

/*
This is the processor code.

The basic design is the same as the csl-next typescript implementation:

The processor takes a style, a bibliography, and a locale, and renders the output.

The primary target is a JSON AST, represented by the ProcTemplateComponent struct.
 */

// TODO: This will need to be generalized later. See:
// https://github.com/bdarcus/csln/issues/105
pub fn refs_to_string(proc_templates: Vec<ProcTemplate>) -> String {
    proc_templates
        .iter()
        .map(|proc_template| {
            proc_template
                .iter()
                .map(|proc_template_component| proc_template_component.to_string())
                .collect::<Vec<String>>()
                .join(". ")
                + "."
        })
        .collect::<Vec<String>>()
        .join("\n\n")
}

/// The processor struct, which takes a style, a bibliography, and a locale, and renders the output.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Processor {
    /// The input style.
    style: Style,
    /// The input bibliography.
    bibliography: Bibliography,
    /// The input citations.
    citations: Vec<Citation>,
    /// The output locale.
    locale: Locale,
}

/// The intermediate representation of a StyleTemplate, which is used to render the output.
pub type ProcTemplate = Vec<ProcTemplateComponent>;

/// The intermediate representation of a StyleTemplateComponent, which is used to render the output.
/// This struct will have two fields: a StyleComponent and a String.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProcTemplateComponent {
    /// The original input style template component, which provides rendering instructions.
    pub template_component: TemplateComponent,
    /// The string to render.
    pub value: String,
    ///
    pub substituted: Option<SubstituteKey>, // FIXME
}

#[test]
fn render_proc_template_component() {
    use csln::style::template::Rendering;
    let template_component = TemplateComponent::SimpleString(TemplateSimpleString {
        variable: Variables::Doi,
        rendering: Some(Rendering {
            emph: Some(true),
            quote: Some(true),
            strong: Some(true),
            prefix: Some("doi: ".to_string()),
            suffix: Some(" ||".to_string()),
            wrap: Some(WrapPunctuation::Parentheses),
        }),
    });
    let value = "10/1234".to_string();
    let proc_template_component = ProcTemplateComponent::new(template_component, value);
    assert_eq!(proc_template_component.to_string(), "(doi: 10/1234 ||)".to_string());
}

impl Display for ProcTemplateComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let rendering = self.template_component.rendering();
        let prefix: String = rendering
            .clone() // REVIEW this compiles, but too much cloning
            .unwrap_or_default()
            .prefix
            .unwrap_or_default();
        let suffix: String =
            rendering.clone().unwrap_or_default().suffix.unwrap_or_default();
        let wrap: WrapPunctuation =
            rendering.unwrap_or_default().wrap.unwrap_or_default();
        let wrap_punct: (String, String) = match wrap {
            WrapPunctuation::None => ("".to_string(), "".to_string()),
            WrapPunctuation::Parentheses => ("(".to_string(), ")".to_string()),
            WrapPunctuation::Brackets => ("[".to_string(), "]".to_string()),
        };
        // REVIEW: is this where to plugin different renderers?
        write!(f, "{}{}{}{}{}", wrap_punct.0, prefix, self.value, suffix, wrap_punct.1)
    }
}

impl ProcTemplateComponent {
    pub fn new(template_component: TemplateComponent, value: String) -> Self {
        ProcTemplateComponent { template_component, value, substituted: None }
    }
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

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema)]
pub struct RenderOptions {
    // Options for the style, including default options.
    global: Config,
    // Options for the citaton or bibliography, that may override the style options.
    local: Config,
    // Locale for the output.
    locale: Locale,
}

/// The intermediate representation of a TemplateComponent, which is used to render the output.
pub trait ProcessComponent<T> {
    fn process(
        &self,
        reference: &InputReference,
        component: &T,
        options: RenderOptions,
    ) -> Option<ProcTemplateComponent>;
}

pub trait ComponentValue {
    fn value(
        &self,
        reference: &InputReference,
        hints: &ProcHints,
        options: &RenderOptions,
    ) -> Option<String>;
}

impl ComponentValue for TemplateComponent {
    fn value(
        &self,
        reference: &InputReference,
        hints: &ProcHints,
        options: &RenderOptions,
        // context: &RenderContext<T>,
    ) -> Option<String> {
        match self {
            TemplateComponent::Title(title) => {
                Some(title.value(reference, hints, options).unwrap_or_default())
            }
            TemplateComponent::Contributor(contributor) => {
                contributor.value(reference, hints, options)
            }
            TemplateComponent::Date(date) => date.value(reference, hints, options),
            TemplateComponent::Number(number) => number.value(reference, hints, options),
            TemplateComponent::SimpleString(string) => {
                string.value(reference, hints, options)
            }
            TemplateComponent::List(_list) => todo!(),
            _ => None,
        }
    }
}

impl ComponentValue for TemplateNumber {
    fn value(
        &self,
        reference: &InputReference,
        _hints: &ProcHints,
        _options: &RenderOptions,
    ) -> Option<String> {
        let number: Option<String> = match &self.number {
            Numbers::Volume => match reference {
                InputReference::SerialComponent(serial_component) => {
                    Some(serial_component.volume.as_ref()?.to_string())
                }
                _ => None,
            },
            Numbers::Issue => match reference {
                InputReference::SerialComponent(serial_component) => {
                    Some(serial_component.issue.as_ref()?.to_string())
                }
                _ => None,
            },
            Numbers::Pages => match reference {
                InputReference::SerialComponent(serial_component) => {
                    Some(serial_component.pages.as_ref()?.to_string())
                }
                InputReference::MonographComponent(monograph_component) => {
                    Some(monograph_component.pages.as_ref()?.to_string())
                }
                _ => None,
            },
        };
        number
    }
}

impl ComponentValue for TemplateSimpleString {
    fn value(
        &self,
        reference: &InputReference,
        _hints: &ProcHints,
        _options: &RenderOptions,
    ) -> Option<String> {
        match self.variable {
            Variables::Doi => match reference {
                InputReference::SerialComponent(serial_component) => {
                    Some(serial_component.doi.as_ref()?.to_string())
                }
                InputReference::MonographComponent(monograph_component) => {
                    Some(monograph_component.doi.as_ref()?.to_string())
                }
                _ => None,
            },
            Variables::Isbn => match reference {
                InputReference::Monograph(monograph_component) => {
                    Some(monograph_component.isbn.as_ref()?.to_string())
                }
                _ => None,
            },
            _ => None, // TODO completes
        }
    }
}

impl ComponentValue for TemplateTitle {
    fn value(
        &self,
        reference: &InputReference,
        _hints: &ProcHints,
        _options: &RenderOptions,
    ) -> Option<String> {
        match &self.title {
            Titles::ParentMonograph => {
                if let InputReference::MonographComponent(monograph_component) = reference
                {
                    Some(monograph_component.parent.title.to_string())
                } else {
                    None
                }
            }
            Titles::ParentSerial => {
                if let InputReference::SerialComponent(serial_component) = reference {
                    Some(serial_component.parent.title.to_string())
                } else {
                    None
                }
            }
            Titles::Primary => {
                if let InputReference::Monograph(monograph) = reference {
                    Some(monograph.title.to_string())
                } else if let InputReference::MonographComponent(monograph_component) =
                    reference
                {
                    Some(monograph_component.title.as_ref()?.to_string())
                } else if let InputReference::SerialComponent(serial_component) =
                    reference
                {
                    Some(serial_component.title.as_ref()?.to_string())
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

// write a test to make sure author substition works
#[test]
fn author_substitution() {
    use csln::bibliography::reference::{Collection, StructuredName};
    let component = TemplateContributor {
        contributor: ContributorRole::Author,
        rendering: None,
        form: csln::style::template::ContributorForm::Long,
    };
    let reference = Collection {
        id: Some("test".to_string()),
        editor: Some(csln::bibliography::reference::Contributor::StructuredName(
            StructuredName {
                family: "Editor".to_string(),
                given: "Jane".to_string(),
            },
        )),
        r#type: csln::bibliography::reference::CollectionType::EditedBook,
        issued: EdtfString("2020".to_string()),
        title: None,
        url: None,
        accessed: None,
        translator: None,
        publisher: None,
        note: None,
        issn: None,
    };
    (assert_eq!(
        component.value(
            &InputReference::Collection(reference),
            &ProcHints::default(),
            &RenderOptions::default()
        ),
        Some("Jane Editor".to_string())
    ));
}

pub fn get_author_substitute(
    reference: &InputReference,
    options: &RenderOptions,
) -> (String, SubstituteKey) {
    let substitute_keys = options.global.substitute.clone().unwrap().template;
    substitute_keys
        .iter()
        .find_map(|key| match key {
            SubstituteKey::Editor => Some((
                reference.editor()?.names(options.global.clone(), false),
                SubstituteKey::Editor,
            )),
            SubstituteKey::Translator => Some((
                reference.translator()?.names(options.global.clone(), false),
                SubstituteKey::Translator,
            )),
            SubstituteKey::Title => {
                Some((reference.title()?.to_string(), SubstituteKey::Title))
            }
        })
        .unwrap()
}

impl ComponentValue for TemplateContributor {
    fn value(
        &self,
        reference: &InputReference,
        _hints: &ProcHints,
        options: &RenderOptions,
    ) -> Option<String> {
        match &self.contributor {
            ContributorRole::Author => match reference.author() {
                Some(author) => Some(author.names(options.global.clone(), false)),
                None => {
                    let substitute = get_author_substitute(reference, options);
                    let substitute_value = substitute.0;
                    // TODO WTD with this? Maybe this needs to be moved to the template?
                    let substitute_key = substitute.1;
                    Some(substitute_value)
                }
            },
            ContributorRole::Editor => {
                Some(reference.editor()?.names(options.global.clone(), false))
            }
            ContributorRole::Translator => {
                Some(reference.translator()?.names(options.global.clone(), false))
            }
            ContributorRole::Publisher => {
                Some(reference.publisher()?.names(options.global.clone(), false))
            }
            ContributorRole::Director => todo!(),
            ContributorRole::Recipient => todo!(),
            ContributorRole::Interviewer => todo!(),
            ContributorRole::Interviewee => todo!(),
            ContributorRole::Composer => todo!(),
            ContributorRole::Inventor => todo!(),
            ContributorRole::Counsel => todo!(),
        }
    }
}

impl ComponentValue for TemplateDate {
    fn value(
        &self,
        reference: &InputReference,
        hints: &ProcHints,
        options: &RenderOptions,
    ) -> Option<String> {
        let locale: &Locale = &options.locale;
        let input_date: EdtfString = match &self.date {
            Dates::Issued => reference.issued()?,
            Dates::OriginalPublished => todo!("original-published"),
            Dates::Accessed => todo!("accessed"),
        };
        let parsed_date = input_date.parse();
        //print!("date form: {:?}", reference.issued);
        let formatted_date: String = match self.form {
            DateForm::Year => parsed_date
                .year() // this line causes a panic if the date is not a year
                .to_string(),
            DateForm::YearMonth => {
                input_date.year_month(locale.dates.months.long.clone())
            }
            DateForm::MonthDay => input_date.month_day(locale.dates.months.long.clone()),
            DateForm::Full => todo!(),
        };

        // TODO: implement this along with localized dates
        fn _config_fmt(options: &RenderOptions) -> DateTimeFormatterOptions {
            match options.global.dates.as_ref().unwrap().month {
                MonthFormat::Long => todo!("long"),
                MonthFormat::Short => todo!("short"),
                MonthFormat::Numeric => todo!("numeric"),
            };
        }

        fn int_to_letter(n: u32) -> String {
            let c = n + 96;
            match char::from_u32(c) {
                Some(ch) => ch.to_string(),
                None => "".to_string(),
            }
        }

        let suffix = if hints.disamb_condition
            // TODO need to check form here also
            // && self.form == style::template::DateForm::Year
            // REVIEW: ugly, and needs to be smarter
            && options.global.processing.clone().unwrap().config().disambiguate.unwrap().year_suffix
            && formatted_date.len() == 4
        {
            int_to_letter((hints.group_index % 26) as u32)
        } else {
            "".to_string()
        };

        Some(formatted_date + &suffix)
    }
}

// #[test]
// fn render_year() {
//     let component = StyleTemplateDate {
//         date: Dates::Issued,
//         form: DateForm::Year,
//         rendering: None,
//     };
//     let reference = InputReference {
//         id: Some("test".to_string()),
//         issued: Some(RefDate::Structured(Edtf::from_str("2020").unwrap())),
//         ..Default::default()
//     };
//     let options = RenderOptions {
//         global: &StyleOptions::default(),
//         local: &StyleOptions::default(),
//     };
//     let rendered_date = component.render(&reference, &ProcHints::default(), &options);
//     assert_eq!(rendered_date, "2020");
// }

impl Processor {
    /// Render references to AST.
    #[inline]
    pub fn process_references(&self) -> Vec<ProcTemplate> {
        let sorted_references = self.sort_references(self.get_references());
        sorted_references
            .par_iter()
            .map(|reference| self.process_reference(reference))
            .collect()
    }

    /// Render a reference to AST.
    fn process_reference(
        &self,
        reference: &InputReference,
    ) -> Vec<ProcTemplateComponent> {
        let bibliography_style = self.style.bibliography.clone();
        self.process_template(reference, bibliography_style.unwrap().template.as_slice())
    }

    fn get_render_options(&self, style: Style, locale: Locale) -> RenderOptions {
        RenderOptions {
            global: style.options.unwrap_or_default(),
            local: Config::default(),
            locale,
        }
    }

    fn process_template(
        &self,
        reference: &InputReference,
        template: &[TemplateComponent],
    ) -> ProcTemplate {
        let mut author_substitution: Option<SubstituteKey> = None;
        template
            .iter()
            .filter_map(|component| {
                // if the below returns a substituted value, set substituted to that
                let proc_template = self.process_template_component(component, reference);
                let substituted = proc_template.as_ref()?.substituted.clone();
                if substituted.is_some() {
                    author_substitution = substituted
                }
                proc_template
            })
            .collect()
    }

    fn process_template_component(
        &self,
        component: &TemplateComponent,
        reference: &InputReference,
    ) -> Option<ProcTemplateComponent> {
        let hints = self.get_proc_hints();
        let reference_id: Option<RefID> = reference.id();
        let hint: ProcHints =
            hints.get(&reference_id.unwrap()).cloned().unwrap_or_default();
        let options = self.get_render_options(self.style.clone(), self.locale.clone());
        let value = component.value(reference, &hint, &options)?; // FIXME substitution
        let substituted = if component.is_author() && reference.author().is_none() {
            Some(SubstituteKey::Editor) // FIXME run a get_substitute_key method here? What about the value below?
        } else {
            None
        };
        let template_component = component.clone();
        // TODO add substitute here, and return the substitueted value if it exists
        if !value.is_empty() {
            Some(ProcTemplateComponent { template_component, value, substituted })
        } else {
            None
        }
    }

    /// Get references from the bibliography.
    pub fn get_references(&self) -> Vec<InputReference> {
        self.bibliography
            .iter()
            .map(|(key, reference)| match reference {
                InputReference::Monograph(monograph) => {
                    let mut input_reference =
                        InputReference::Monograph(monograph.clone());
                    input_reference.set_id(key.clone());
                    input_reference
                }
                InputReference::MonographComponent(monograph_component) => {
                    let mut input_reference =
                        InputReference::MonographComponent(monograph_component.clone());
                    input_reference.set_id(key.clone());
                    input_reference
                }
                InputReference::SerialComponent(serial_component) => {
                    let mut input_reference =
                        InputReference::SerialComponent(serial_component.clone());
                    input_reference.set_id(key.clone());
                    input_reference
                }
                InputReference::Collection(collection) => {
                    let mut input_reference =
                        InputReference::Collection(collection.clone());
                    input_reference.set_id(key.clone());
                    input_reference
                }
            })
            .collect()
    }

    /// Get a reference from the bibliography by id/citekey.
    pub fn get_reference(&self, id: &str) -> Option<InputReference> {
        self.bibliography.get(id).cloned()
    }

    pub fn get_cited_references(&self) -> Vec<InputReference> {
        let mut cited_references = Vec::new();
        for key in &self.get_cited_keys() {
            let reference = self.get_reference(key);
            if let Some(reference) = reference {
                cited_references.push(reference);
            }
        }
        cited_references
    }

    /// Return a list of all the keys cited in the document, in order.
    pub fn get_cited_keys(&self) -> Vec<String> {
        self.citations
            .iter()
            .flat_map(|c| {
                c.references
                    .iter()
                    .map(|cr| cr.ref_id.clone())
                    .collect::<Vec<String>>()
            })
            .collect()
    }

    /// Sort the references according to instructions in the style.
    #[inline]
    pub fn sort_references(
        &self,
        references: Vec<InputReference>,
    ) -> Vec<InputReference> {
        let mut references: Vec<InputReference> = references;
        let options: Config = self.style.options.clone().unwrap_or_default();
        if let Some(sort_config) =
            options.processing.clone().unwrap_or_default().config().sort
        {
            sort_config.template.iter().rev().for_each(|sort| match sort.key {
                SortKey::Author => {
                    references.par_sort_by(|a, b| {
                        let a_author = match a.author() {
                            Some(author) => author.names(options.clone(), true),
                            None => return Ordering::Equal,
                        };
                        let b_author = match b.author() {
                            Some(author) => author.names(options.clone(), true),
                            None => return Ordering::Equal,
                        };
                        a_author.to_lowercase().cmp(&b_author.to_lowercase())
                    });
                }
                SortKey::Year => {
                    references.par_sort_by(|a: &InputReference, b: &InputReference| {
                        let a_year = a.issued().as_ref().unwrap().year();
                        let b_year = b.issued().as_ref().unwrap().year();
                        b_year.cmp(&a_year)
                    });
                }
                _ => {}
            });
        }
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
                        let ref_id = match reference {
                            InputReference::Monograph(monograph) => monograph.id.clone(),
                            InputReference::MonographComponent(monograph_component) => {
                                monograph_component.id.clone()
                            }
                            InputReference::SerialComponent(serial_component) => {
                                serial_component.id.clone()
                            }
                            InputReference::Collection(collection) => {
                                collection.id.clone()
                            }
                        };
                        (ref_id.unwrap(), proc_hint)
                    },
                )
            })
            .collect();
        proc_hints
    }

    /// Return a string to use for grouping for a given reference, using instructions in the style.
    fn make_group_key(&self, reference: &InputReference) -> String {
        let options: csln::style::options::Config = match self.style.options {
            Some(ref options) => options.clone(),
            None => Config::default(), // TODO is this right?
        };
        let group_config = options.processing.unwrap_or_default().config().group.unwrap();
        let options = self.style.options.clone();
        let as_sorted = false;
        let group_key = group_config
            .template
            // This is likely unnecessary, but just in case.
            .par_iter()
            .map(|key| match key {
                SortKey::Author => match reference.author() {
                    Some(author) => author.names(options.clone().unwrap(), as_sorted),
                    None => "".to_string(),
                },
                SortKey::Year => {
                    reference.issued().as_ref().unwrap().parse().year().to_string()
                }
                SortKey::Title => reference.title().as_ref().unwrap().to_string(),
                _ => "".to_string(), // REVIEW is this right?
            })
            .collect::<Vec<String>>()
            .join(":");
        group_key
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

    pub fn new(
        style: Style,
        bibliography: Bibliography,
        citations: Vec<Citation>,
        locale: Locale,
    ) -> Processor {
        Processor { style, bibliography, citations, locale }
    }
}
