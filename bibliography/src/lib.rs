use std::collections::HashMap;

mod reference;
pub use reference::InputReference;

/// A bibliography is a collection of references.
pub type InputBibliography = HashMap<String, InputReference>;