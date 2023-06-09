/* 
This is a small module for defining basic data types and functions for formatting.

Some of the ideas and code are adapted from the `typst-hayagriva` crate.
 */

use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use url::Url;
use unic_langid::LanguageIdentifier;
use bibliography::reference::Contributor;
use style::options::StyleOptions;

/* 
This section almost-completely adapted from HayaGriva.

The primary differences:

1. We use `serde` for serialization and deserialization.
2. A more general notion of Contributor.
3. Date is a string, not a struct; either EDTF or literal.
4. Use a different model for the `Entry` struct.
 */

/// The data types that can possibly be held by the various fields of an
/// [`InputReference`].
#[derive(Clone, Debug, Display, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Value {
    /// A [Title] containing a canonical value and optionally translations and
    /// shorthands, all of which are formattable.
    Title(Title),
    /// A [FmtString] with which the user can override various
    /// automatic formatters.
    FmtString(FmtString),
    /// A string to be reproduced as-is.
    Text(String),
    /// An integer.
    Integer(i64),
    /// A date string, either EDTF or literal.
    Date(Date),
    /// A Contributor.  The model allows single strings, a person names struct, or a list of either.
    Contributor(Contributor),
    /// This could be both an Integer or a Number.
    IntegerOrText(NumOrStr),
    /// A range between two integers.
    Range(std::ops::Range<i64>),
    /// A duration (of a song or an performance for example).
    Duration(Duration),
    /// A part of a period.
    TimeRange(std::ops::Range<Duration>),
    /// An [URL, possibly with a date of when it has been consulted](QualifiedUrl).
    Url(Url),
    /// A [Unicode Language Identifier](LanguageIdentifier).
    Language(LanguageIdentifier),
}

/// A value that could be either a number or a string.
#[derive(Clone, Debug, Display, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum NumOrStr {
    /// It's a number!
    Number(i64),
    /// It's a string!
    Str(String),
}

impl Display for NumOrStr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Number(i) => write!(f, "{}", i),
            Self::Str(s) => write!(f, "{}", s),
        }
    }
}

impl From<NumOrStr> for String {
    fn from(num: NumOrStr) -> Self {
        num.to_string()
    }
}

impl From<NumOrStr> for i64 {
    fn from(num: NumOrStr) -> Self {
        match num {
            NumOrStr::Number(i) => i,
            NumOrStr::Str(s) => s.parse().unwrap_or(0),
        }
    }
}



