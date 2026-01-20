/*
SPDX-License-Identifier: MPL-2.0
SPDX-FileCopyrightText: © 2023-2026 Bruce D'Arcus
*/

use crate::types::{ProcTemplate, ProcTemplateComponent};
use csln::style::template::WrapPunctuation;
use std::fmt::{self, Display, Formatter};

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
        // Also, how to handle the different affixes, including within the values?
        let result = wrap_punct.0
            + &prefix
            + &self.values.prefix.clone().unwrap_or_default()
            + &self.values.value
            + &self.values.suffix.clone().unwrap_or_default()
            + &suffix
            + &wrap_punct.1;
        write!(f, "{}", result)
    }
}

#[test]
fn render_proc_template_component() {
    use crate::types::ProcValues;
    use csln::style::template::{TemplateComponent, TemplateSimpleString, Rendering, Variables};
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
    let proc_template_component = ProcTemplateComponent::new(
        template_component,
        ProcValues { value, prefix: None, suffix: None },
    );
    assert_eq!(proc_template_component.to_string(), "(doi: 10/1234 ||)".to_string());
}
