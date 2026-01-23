/*
SPDX-License-Identifier: MPL-2.0
SPDX-FileCopyrightText: © 2023-2026 Bruce D'Arcus
*/

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Rendering instructions for a template component.
#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
pub struct Rendering {
    pub emph: Option<bool>,
    pub quote: Option<bool>,
    pub strong: Option<bool>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub wrap: Option<WrapPunctuation>,
}

/// The punctuation to wrap a template component in.
#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum WrapPunctuation {
    Parentheses,
    Brackets,
    #[default]
    None,
}

/// The Template component model. Each item is for a specific datatype.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(untagged)]
#[non_exhaustive]
pub enum TemplateComponent {
    Contributor(TemplateContributor),
    Date(TemplateDate),
    List(TemplateList),
    Title(TemplateTitle),
    Number(TemplateNumber),
    SimpleString(TemplateSimpleString),
}

impl TemplateComponent {
    pub fn rendering(&self) -> Option<Rendering> {
        match self {
            TemplateComponent::Contributor(c) => c.rendering.clone(),
            TemplateComponent::Date(d) => d.rendering.clone(),
            TemplateComponent::List(_l) => None,
            TemplateComponent::Title(t) => t.rendering.clone(),
            TemplateComponent::Number(n) => n.rendering.clone(),
            TemplateComponent::SimpleString(s) => s.rendering.clone(),
        }
    }

    // TODO do I need this?
    pub fn is_author(&self) -> bool {
        match self {
            TemplateComponent::Contributor(c) => c.contributor == ContributorRole::Author,
            _ => false,
        }
    }
}

/// A simple string component, to render a string variable.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
pub struct TemplateSimpleString {
    pub variable: Variables,
    pub rendering: Option<Rendering>,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Variables {
    // TODO: add more variables
    Doi,
    Isbn,
    Issn,
}

/// A number component, to render a number.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
pub struct TemplateNumber {
    pub number: Numbers,
    pub form: Option<NumberForm>,
    pub rendering: Option<Rendering>,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Numbers {
    Volume,
    Issue,
    Pages,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NumberForm {
    #[default]
    Numeric,
    Ordinal,
}

/// To render is a list of more than one item; primarily to enable use of a delimiter to join the items.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
pub struct TemplateList {
    pub delimiter: Option<DelimiterPunctuation>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub wrap: Option<WrapPunctuation>,
    pub items: Vec<TemplateComponent>,
}

/// The punctuation to use as a delimiter between items in a list.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum DelimiterPunctuation {
    Comma,
    Semicolon,
    Period,
    Colon,
    Ampersand,
    VerticalLine,
    Slash,
    Hyphen,
    Space,
    None,
}

/// A contributor component, to render a list of contributors.
// TODO incomplete
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
pub struct TemplateContributor {
    pub contributor: ContributorRole,
    pub form: ContributorForm,
    pub rendering: Option<Rendering>,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ContributorForm {
    Long,
    Short,
    Verb,
    VerbShort,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum ContributorRole {
    Author,
    Editor,
    Translator,
    Director,
    Publisher,
    Recipient,
    Interviewer,
    Interviewee,
    Inventor,
    Counsel,
    Composer,
}

/// A date component, to render a date.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
pub struct TemplateDate {
    pub date: Dates,
    pub form: DateForm,
    pub rendering: Option<Rendering>,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Dates {
    Issued,
    Accessed,
    OriginalPublished,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum DateForm {
    Year,
    YearMonth,
    Full,
    MonthDay,
}

/// A title component, to render a title.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
pub struct TemplateTitle {
    pub title: Titles,
    pub form: Option<TitleForm>,
    pub rendering: Option<Rendering>,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum Titles {
    /// The primary title for the cited work.
    Primary,
    /// The title of a book or other monograph that the cited work is a part of.
    ParentMonograph,
    /// The titles of a periodical or other serial that the cited work is a part of.
    ParentSerial,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TitleForm {
    Short,
    Long,
}
