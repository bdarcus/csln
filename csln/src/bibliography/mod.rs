use std::collections::HashMap;

/*
SPDX-License-Identifier: MPL-2.0
SPDX-FileCopyrightText: © 2023-2026 Bruce D'Arcus
*/

pub mod reference;
pub use reference::InputReference;

/// A bibliography is a collection of references.
pub type InputBibliography = HashMap<String, InputReference>;
