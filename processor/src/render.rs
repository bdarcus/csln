/*
SPDX-License-Identifier: MPL-2.0
SPDX-FileCopyrightText: © 2023-2026 Bruce D'Arcus
*/

use crate::types::{ProcTemplate, ProcTemplateComponent};
use csln::style::template::WrapPunctuation;
use std::fmt::{self, Display, Formatter};

// TODO: This will need to be generalized later. See:
// https://github.com/bdarcus/csln/issues/105
use std::fmt::Write;

/// Render processed templates into a final string.
pub fn refs_to_string(proc_templates: Vec<ProcTemplate>) -> String {
    let mut output = String::new();
    // Optimized: Use a loop and write! to avoid multiple intermediate string allocations
    // that would occur with a map/collect/join chain.
    for (i, proc_template) in proc_templates.iter().enumerate() {
        if i > 0 {
            output.push_str("\n\n");
        }
        for (j, component) in proc_template.iter().enumerate() {
            if j > 0 {
                output.push_str(". ");
            }
            let _ = write!(&mut output, "{}", component);
        }
        output.push('.');
    }
    output
}

impl Display for ProcTemplateComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let rendering = self.template_component.rendering();
        let r = rendering.as_ref();

        let prefix = r.and_then(|r| r.prefix.as_deref()).unwrap_or_default();
        let suffix = r.and_then(|r| r.suffix.as_deref()).unwrap_or_default();
        let wrap = r.and_then(|r| r.wrap.as_ref()).unwrap_or(&WrapPunctuation::None);

        let wrap_punct: (&str, &str) = match wrap {
            WrapPunctuation::None => ("", ""),
            WrapPunctuation::Parentheses => ("(", ")"),
            WrapPunctuation::Brackets => ("[", "]"),
        };

        write!(
            f,
            "{}{}{}{}{}{}{}",
            wrap_punct.0,
            prefix,
            self.values.prefix.as_deref().unwrap_or_default(),
            self.values.value,
            self.values.suffix.as_deref().unwrap_or_default(),
            suffix,
            wrap_punct.1
        )
    }
}

#[test]
fn render_proc_template_component() {
    use crate::types::ProcValues;
    use csln::style::template::{
        Rendering, TemplateComponent, TemplateSimpleString, Variables,
    };
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
