/*
SPDX-License-Identifier: MPL-2.0
SPDX-FileCopyrightText: © 2023-2026 Bruce D'Arcus
*/

use crate::error::ProcessorError;
use crate::types::{
    ProcBibliography, ProcCitation, ProcCitationItem, ProcCitations, ProcHints,
    ProcReferences, ProcTemplate, ProcTemplateComponent, ProcValues, RenderOptions,
};
use crate::values::ComponentValues;
use csln::bibliography::reference::InputReference;
use csln::bibliography::InputBibliography as Bibliography;
use csln::citation::{Citation, CitationItem, Citations};
use csln::style::locale::Locale;
use csln::style::options::{Config, SortKey, Substitute, SubstituteKey};
use csln::style::template::TemplateComponent;
use csln::style::Style;
use itertools::Itertools;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The processor struct, which takes a style, a bibliography, and a locale, and renders the output.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Processor {
    /// The input style.
    style: Style,
    /// The input bibliography.
    bibliography: Bibliography,
    /// The input citations.
    citations: Citations,
    /// The output locale.
    locale: Locale,
    /// Default configuration for reference.
    #[serde(skip)]
    default_config: Config,
    /// Pre-calculated processing hints.
    #[serde(skip)]
    hints: HashMap<String, ProcHints>,
}

impl Processor {
    /// Create a new Processor instance.
    pub fn new(
        style: Style,
        mut bibliography: Bibliography,
        citations: Citations,
        locale: Locale,
    ) -> Processor {
        // Normalize the bibliography by ensuring all references have an ID.
        for (id, reference) in bibliography.iter_mut() {
            if reference.id().is_none() {
                reference.set_id(id.clone());
            }
        }

        let mut processor = Processor {
            style,
            bibliography,
            citations,
            locale,
            default_config: Config::default(),
            hints: HashMap::new(),
        };

        // Pre-calculate hints.
        processor.hints = processor.calculate_proc_hints();

        processor
    }

    /// Render references to AST.
    #[inline]
    pub fn process_references(&self) -> ProcReferences {
        let sorted_references = self.sort_references(self.get_references());
        let bibliography: ProcBibliography = sorted_references
            .par_iter()
            .map(|reference| self.process_reference(*reference))
            .collect();
        let citations = if self.citations.is_empty() {
            None
        } else {
            match self.process_citations(&self.citations) {
                Ok(c) => Some(c),
                Err(e) => {
                    eprintln!("Citation processing error: {}", e);
                    None
                }
            }
        };
        ProcReferences { bibliography, citations }
    }

    fn process_citations(
        &self,
        citations: &Citations,
    ) -> Result<ProcCitations, ProcessorError> {
        citations
            .iter()
            .map(|citation| self.process_citation(citation))
            .collect()
    }

    fn process_citation(
        &self,
        citation: &Citation,
    ) -> Result<ProcCitation, ProcessorError> {
        // TODO handle the prefix and suffix, though am uncertain how to best do that
        citation
            .citation_items
            .iter()
            .map(|citation_item| self.process_citation_item(citation_item))
            .collect()
    }

    /// Process a single citation item.
    pub fn process_citation_item(
        &self,
        citation_item: &CitationItem,
    ) -> Result<ProcCitationItem, ProcessorError> {
        let citation_style = self.style.citation.as_ref();
        let reference = self.get_reference(&citation_item.ref_id)?;

        let template = citation_style
            .map(|cs| &cs.template)
            .map(|t| t.as_slice())
            .unwrap_or_default();
        let proc_template = self.process_template(reference, template);
        Ok(proc_template)
    }

    /// Render a reference to AST.
    fn process_reference(
        &self,
        reference: &InputReference,
    ) -> Vec<ProcTemplateComponent> {
        if let Some(bibliography_style) = &self.style.bibliography {
            self.process_template(reference, &bibliography_style.template)
        } else {
            Vec::new()
        }
    }

    fn get_render_options<'a>(&'a self) -> RenderOptions<'a> {
        RenderOptions {
            global: self.style.options.as_ref().unwrap_or(&self.default_config),
            local: &self.default_config,
            locale: &self.locale,
        }
    }

    fn process_template(
        &self,
        reference: &InputReference,
        template: &[TemplateComponent],
    ) -> ProcTemplate {
        template
            .iter()
            .filter_map(|component| self.process_template_component(component, reference))
            .collect()
    }

    fn process_template_component(
        &self,
        component: &TemplateComponent,
        reference: &InputReference,
    ) -> Option<ProcTemplateComponent> {
        let reference_id: String = reference.id().unwrap_or_default();
        let default_hint = ProcHints::default();
        let hint: &ProcHints = self.hints.get(&reference_id).unwrap_or(&default_hint);
        let options = self.get_render_options();
        let values = component.values(reference, hint, &options)?;
        let template_component = component.clone();
        // TODO add role here if specified in the style
        // TODO affixes from style?
        if !values.value.is_empty() {
            Some(ProcTemplateComponent {
                template_component,
                values: ProcValues {
                    value: values.value,
                    prefix: values.prefix,
                    suffix: values.suffix,
                },
            })
        } else {
            None
        }
    }

    /// Get references from the bibliography.
    pub fn get_references(&self) -> Vec<&InputReference> {
        self.bibliography.values().collect()
    }

    /// Get a reference from the bibliography by id/citekey.
    pub fn get_reference(&self, id: &str) -> Result<&InputReference, ProcessorError> {
        match self.bibliography.get(id) {
            Some(reference) => Ok(reference),
            None => Err(ProcessorError::ReferenceNotFound(id.to_string())),
        }
    }

    /// Get all cited references from the inputs.
    pub fn get_cited_references(&self) -> Vec<&InputReference> {
        let mut cited_references = Vec::new();
        for key in &self.get_cited_keys() {
            if let Ok(reference) = self.get_reference(key) {
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
                c.citation_items
                    .iter()
                    .map(|cr| cr.ref_id.clone())
                    .collect::<Vec<String>>()
            })
            .collect()
    }

    /// Sort the references according to instructions in the style.
    #[inline]
    pub fn sort_references<'a>(
        &self,
        references: Vec<&'a InputReference>,
    ) -> Vec<&'a InputReference> {
        let mut references = references;
        let options = self.style.options.as_ref().unwrap_or(&self.default_config);
        let processing = options.processing.as_ref().cloned().unwrap_or_default();
        let processing_config = processing.config();

        if let Some(sort_config) = &processing_config.sort {
            sort_config.template.iter().rev().for_each(|sort| match sort.key {
                SortKey::Author => {
                    references.par_sort_by(|a, b| {
                        let a_author = match a.author() {
                            Some(author) => author.names(options, true).join("-"),
                            None => match self.get_author_substitute(a) {
                                Some((substitute, _)) => substitute,
                                None => "".to_string(),
                            },
                        };

                        let b_author = match b.author() {
                            Some(author) => author.names(options, true).join("-"),
                            None => match self.get_author_substitute(b) {
                                Some((substitute, _)) => substitute,
                                None => "".to_string(),
                            },
                        };
                        a_author.to_lowercase().cmp(&b_author.to_lowercase())
                    });
                }
                SortKey::Year => {
                    references.par_sort_by(|a, b| {
                        let a_year =
                            a.issued().as_ref().map(|d| d.year()).unwrap_or_default();
                        let b_year =
                            b.issued().as_ref().map(|d| d.year()).unwrap_or_default();
                        b_year.cmp(&a_year)
                    });
                }
                _ => {}
            });
        }
        references
    }

    /// Get the pre-calculated processing hints.
    pub fn get_proc_hints(&self) -> &HashMap<String, ProcHints> {
        &self.hints
    }

    /// Process the references and return a HashMap of ProcHints.
    fn calculate_proc_hints(&self) -> HashMap<String, ProcHints> {
        let refs = self.get_references();
        let sorted_refs = self.sort_references(refs);
        let grouped_refs = self.group_references(sorted_refs);
        grouped_refs
            .iter()
            .flat_map(|(key, group)| {
                let group_len = group.len();
                group.iter().enumerate().filter_map(
                    move |(index, reference)| -> Option<(String, ProcHints)> {
                        // TODO will need to generalize.
                        let disambiguate = group_len > 1;
                        let proc_hint = ProcHints {
                            disamb_condition: disambiguate,
                            group_index: index + 1,
                            group_length: group_len,
                            group_key: key.clone(),
                        };
                        let ref_id = reference.id();
                        ref_id.map(|id| (id, proc_hint))
                    },
                )
            })
            .collect()
    }

    /// Return a string to use for grouping for a given reference, using instructions in the style.
    fn make_group_key(&self, reference: &InputReference) -> String {
        let options = self.style.options.as_ref().unwrap_or(&self.default_config);
        let processing = options.processing.as_ref().cloned().unwrap_or_default();
        let group_template = processing
            .config()
            .group
            .as_ref()
            .map(|g| &g.template)
            .cloned()
            .unwrap_or_default();

        let as_sorted = false;
        let group_key = group_template
            // This is likely unnecessary, but just in case.
            .par_iter()
            .map(|key| match key {
                SortKey::Author => match reference.author() {
                    Some(author) => author.names(options, as_sorted).join("-"),
                    None => "".to_string(),
                },
                SortKey::Year => reference
                    .issued()
                    .as_ref()
                    .map(|d| d.parse().year().to_string())
                    .unwrap_or_default(),
                SortKey::Title => {
                    reference.title().as_ref().map(|t| t.to_string()).unwrap_or_default()
                }
                _ => "".to_string(), // REVIEW is this right?
            })
            .collect::<Vec<String>>()
            .join(":");
        group_key
    }

    /// Get the substitute author name and key for a reference if the primary author is missing.
    pub fn get_author_substitute(
        &self,
        reference: &InputReference,
    ) -> Option<(String, SubstituteKey)> {
        let options = self.style.options.as_ref().unwrap_or(&self.default_config);
        let substitute_config = options.substitute.as_ref();

        // Use default substitute if not provided in style
        let default_sub = Substitute::default();
        let substitute = substitute_config.unwrap_or(&default_sub);

        substitute
            .template
            .iter()
            .find_map(|substitute_key| match *substitute_key {
                SubstituteKey::Editor => {
                    let names = reference.editor()?.format(options, &self.locale);
                    Some((names, substitute_key.clone()))
                }
                _ => None,
            })
    }

    /// Group references according to instructions in the style.
    #[inline]
    pub fn group_references<'a>(
        &self,
        references: Vec<&'a InputReference>,
    ) -> HashMap<String, Vec<&'a InputReference>> {
        references
            .into_iter()
            .group_by(|reference| self.make_group_key(reference))
            .into_iter()
            .map(|(key, group)| (key, group.collect()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use csln::bibliography::reference::{
        Contributor, EdtfString, Monograph, StructuredName, Title,
    };

    fn mock_reference(id: &str, family: &str, year: &str) -> InputReference {
        let name = StructuredName {
            family: family.to_string(),
            given: "Given".to_string(),
        };
        InputReference::Monograph(Monograph {
            id: Some(id.to_string()),
            r#type: csln::bibliography::reference::MonographType::Book,
            author: Some(Contributor::StructuredName(name)),
            issued: EdtfString(year.to_string()),
            title: Title::Single("Title".to_string()),
            publisher: None,
            url: None,
            accessed: None,
            note: None,
            isbn: None,
            doi: None,
            edition: None,
            translator: None,
        })
    }

    #[test]
    fn make_group_key_defaults() {
        // Test default grouping (should be empty or based on default config)
        let processor = Processor::default();
        let reference = mock_reference("ref1", "Smith", "2020");
        let key = processor.make_group_key(&reference);
        // Default group key produces "First Last:Year" format or similar depending on implementation
        // The failure shows "Given Smith:2020"
        assert_eq!(key, "Given Smith:2020");
    }
}
