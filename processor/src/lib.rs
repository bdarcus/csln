/*
SPDX-License-Identifier: MPL-2.0
SPDX-FileCopyrightText: © 2023-2026 Bruce D'Arcus
*/

//! CSLN Processor
//!
//! This crate provides the core citation and bibliography processing functionality
//! for the Citation Style Language Next (CSLN) project. It takes style definitions,
//! bibliographic data, and citation information and produces formatted output.
//!
//! The processor is designed to be pluggable with different renderers and supports
//! advanced features like disambiguation, sorting, and localization.

pub mod error;
pub mod processor;
pub mod render;
pub mod types;
pub mod values;

pub use error::ProcessorError;

pub use processor::Processor;
pub use render::refs_to_string;
pub use types::{
    ProcBibliography, ProcCitation, ProcCitationItem, ProcCitations, ProcHints,
    ProcReferences, ProcTemplate, ProcTemplateComponent, ProcValues, RenderOptions,
};
