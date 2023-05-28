use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::option::Option;

use crate::bibliography::InputBibliography as Bibliography;
use crate::bibliography::InputReference;
use crate::style::options::{StyleOptions, StyleSorting};
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
    proc_value: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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
    fn get_proc_references(&self) -> Vec<ProcReference> {
        // here return a vector of ProcReference structs from the bibliography
        // use iter and map to construct the vector
        // for each reference in the bibliography, construct a ProcReference
        self.bibliography
            .values()
            .cloned()
            .map(|input_reference| ProcReference {
                data: input_reference,
                proc_hints: None,
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
}
