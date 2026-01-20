/*
SPDX-License-Identifier: MPL-2.0
SPDX-FileCopyrightText: © 2023 Bruce D'Arcus
*/



use csln::style::locale::Locale;
use csln::style::options::Config;
use csln::style::template::TemplateComponent;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


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
    pub values: ProcValues,
}

impl ProcTemplateComponent {
    pub fn new(template_component: TemplateComponent, values: ProcValues) -> Self {
        ProcTemplateComponent { template_component, values }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
/// Holds one or more processed strings, ready for final rendering.
pub struct ProcValues {
    /// The primary string to render.
    pub value: String,
    /// The prefix to render.
    pub prefix: Option<String>,
    /// The suffix to render.
    pub suffix: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "kebab-case")]
/// Holds the intermediate processing hints for a reference that can be used
/// to render the output; particularly for disambiguation.
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
/// Configuration options.
pub struct RenderOptions {
    // Options for the style, including default options.
    pub global: Config,
    // Options for the citaton or bibliography, that may override the style options.
    pub local: Config,
    // Locale for the output.
    pub locale: Locale,
}

/// The intermediate representation of rendered citations and bibliography.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct ProcReferences {
    pub bibliography: ProcBibliography,
    /// Process the citations, if there are any.
    pub citations: Option<ProcCitations>,
}

pub type ProcBibliography = Vec<ProcTemplate>;
pub type ProcCitationItem = Vec<ProcTemplateComponent>;
pub type ProcCitation = Vec<ProcCitationItem>;
pub type ProcCitations = Vec<ProcCitation>;
