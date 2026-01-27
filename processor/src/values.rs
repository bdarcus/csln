/*
SPDX-License-Identifier: MPL-2.0
SPDX-FileCopyrightText: © 2023-2026 Bruce D'Arcus
*/

use crate::types::{ProcHints, ProcValues, RenderOptions};
use csln::bibliography::reference::{EdtfString, InputReference};
use csln::style::locale::Locale;
use csln::style::options::MonthFormat;
use csln::style::template::{
    ContributorForm, ContributorRole, DateForm, Dates, Numbers, TemplateComponent,
    TemplateContributor, TemplateDate, TemplateNumber, TemplateSimpleString,
    TemplateTitle, Titles, Variables,
};
use icu::datetime::DateTimeFormatterOptions;

/// Trait to extract values from template components based on reference data.
pub trait ComponentValues {
    fn values(
        &self,
        reference: &InputReference,
        hints: &ProcHints,
        options: &RenderOptions<'_>,
    ) -> Option<ProcValues>;
}

impl ComponentValues for TemplateComponent {
    fn values(
        &self,
        reference: &InputReference,
        hints: &ProcHints,
        options: &RenderOptions<'_>,
    ) -> Option<ProcValues> {
        let proc_values = match self {
            TemplateComponent::Title(title) => title.values(reference, hints, options),
            TemplateComponent::Contributor(contributor) => {
                contributor.values(reference, hints, options)
            }
            TemplateComponent::Date(date) => date.values(reference, hints, options),
            TemplateComponent::Number(number) => number.values(reference, hints, options),
            TemplateComponent::SimpleString(string) => {
                string.values(reference, hints, options)
            }
            TemplateComponent::List(_list) => todo!(),
            _ => None,
        };
        Some(ProcValues {
            value: proc_values.as_ref()?.value.clone(),
            prefix: proc_values.as_ref()?.prefix.clone(),
            suffix: proc_values.as_ref()?.suffix.clone(),
        })
    }
}

impl ComponentValues for TemplateNumber {
    fn values(
        &self,
        reference: &InputReference,
        _hints: &ProcHints,
        _options: &RenderOptions<'_>,
    ) -> Option<ProcValues> {
        let number: Option<String> = match &self.number {
            Numbers::Volume => match reference {
                InputReference::SerialComponent(serial_component) => {
                    Some(serial_component.volume.as_ref()?.to_string())
                }
                _ => None,
            },
            Numbers::Issue => match reference {
                InputReference::SerialComponent(serial_component) => {
                    Some(serial_component.issue.as_ref()?.to_string())
                }
                _ => None,
            },
            Numbers::Pages => match reference {
                InputReference::SerialComponent(serial_component) => {
                    Some(serial_component.pages.as_ref()?.to_string())
                }
                InputReference::CollectionComponent(monograph_component) => {
                    Some(monograph_component.pages.as_ref()?.to_string())
                }
                _ => None,
            },
        };
        Some(ProcValues {
            value: number.unwrap_or_default(),
            prefix: None,
            suffix: None,
        })
    }
}

impl ComponentValues for TemplateSimpleString {
    fn values(
        &self,
        reference: &InputReference,
        _hints: &ProcHints,
        _options: &RenderOptions<'_>,
    ) -> Option<ProcValues> {
        let value = match self.variable {
            Variables::Doi => match reference {
                InputReference::SerialComponent(serial_component) => {
                    Some(serial_component.doi.as_ref()?.to_string())
                }
                InputReference::CollectionComponent(monograph_component) => {
                    Some(monograph_component.doi.as_ref()?.to_string())
                }
                _ => None,
            },
            Variables::Isbn => match reference {
                InputReference::Monograph(monograph_component) => {
                    Some(monograph_component.isbn.as_ref()?.to_string())
                }
                _ => None,
            },
            _ => None, // TODO completes
        };
        Some(ProcValues {
            value: value.unwrap_or_default(),
            prefix: None,
            suffix: None,
        })
    }
}

impl ComponentValues for TemplateTitle {
    fn values(
        &self,
        reference: &InputReference,
        _hints: &ProcHints,
        _options: &RenderOptions<'_>,
    ) -> Option<ProcValues> {
        let value = match &self.title {
            Titles::ParentMonograph => {
                if let InputReference::CollectionComponent(collection_component) =
                    reference
                {
                    Some(collection_component.parent.title.as_ref()?.to_string())
                } else {
                    None
                }
            }
            Titles::ParentSerial => {
                if let InputReference::SerialComponent(serial_component) = reference {
                    Some(serial_component.parent.title.to_string())
                } else {
                    None
                }
            }
            Titles::Primary => match reference {
                InputReference::Monograph(monograph) => Some(monograph.title.to_string()),
                InputReference::Collection(collection) => {
                    Some(collection.title.as_ref()?.to_string())
                }
                InputReference::CollectionComponent(monograph_component) => {
                    Some(monograph_component.title.as_ref()?.to_string())
                }
                InputReference::SerialComponent(serial_component) => {
                    Some(serial_component.title.as_ref()?.to_string())
                }
            },
            _ => None,
        };
        Some(ProcValues {
            value: value.unwrap_or_default(),
            prefix: None,
            suffix: None,
        })
    }
}

/// Convert a contributor role to its string representation.
pub fn role_to_string(
    role: &ContributorRole,
    locale: &Locale,
    form: ContributorForm,
    length: usize,
) -> Option<String> {
    let term = locale.roles.get(role)?; // FIXME causes panic
    match form {
        ContributorForm::Long => {
            if length > 1 {
                Some(term.plural.long.clone())
            } else {
                Some(term.singular.long.clone())
            }
        }
        ContributorForm::Short => {
            if length > 1 {
                Some(term.plural.short.clone())
            } else {
                Some(term.singular.short.clone())
            }
        }
        ContributorForm::Verb => Some(term.verb.long.clone()),
        ContributorForm::VerbShort => Some(term.verb.short.clone()),
    }
}

#[test]
fn role_form_to_string() {
    use csln::style::locale::{ContributorTerm, Locale, SimpleTerm};
    let mut locale = Locale::default();
    locale.roles.insert(
        ContributorRole::Editor,
        ContributorTerm {
            singular: SimpleTerm {
                long: "editor".to_string(),
                short: "ed".to_string(),
            },
            plural: SimpleTerm {
                long: "editors".to_string(),
                short: "eds".to_string(),
            },
            verb: SimpleTerm {
                long: "edited by".to_string(),
                short: "ed".to_string(),
            },
        },
    );
    let role = ContributorRole::Editor;
    let form = ContributorForm::Long;
    let length = 1;
    let result = role_to_string(&role, &locale, form, length);
    assert_eq!(result, Some("editor".to_string()));
}

impl ComponentValues for TemplateContributor {
    fn values(
        &self,
        reference: &InputReference,
        _hints: &ProcHints,
        options: &RenderOptions<'_>,
    ) -> Option<ProcValues> {
        let locale = options.locale;
        match &self.contributor {
            ContributorRole::Author => {
                let author = reference.author();
                if author.is_some() {
                    Some(ProcValues {
                        value: author?.format(options.global, locale),
                        prefix: None,
                        suffix: None,
                    })
                } else {
                    // TODO generalize the substitution
                    let add_role_form = options
                        .global
                        .substitute
                        .as_ref()
                        .and_then(|s| s.contributor_role_form.clone());
                    let editor = reference.editor()?;
                    let editor_length = editor.names(options.global, true).len();
                    // get the role string; if it's in fact author, it will be None
                    let suffix = add_role_form.map(|role_form| {
                        role_to_string(
                            &ContributorRole::Editor,
                            locale,
                            role_form,
                            editor_length,
                        )
                    });
                    let suffix_padded =
                        suffix.and_then(|s| s.map(|val| format!(" {}", val))); // TODO fix this matching logic

                    Some(ProcValues {
                        value: editor.format(options.global, locale),
                        prefix: None,
                        suffix: suffix_padded,
                    })
                }
            }
            ContributorRole::Editor => {
                match reference {
                    &InputReference::Collection(_) => None,
                    _ => {
                        let editor = &reference.editor()?;
                        let form = &self.form;
                        let editor_length = editor.names(options.global, true).len();
                        // TODO handle verb and non-verb forms

                        match form {
                            ContributorForm::Verb | ContributorForm::VerbShort => {
                                let prefix = role_to_string(
                                    &self.contributor,
                                    locale,
                                    form.clone(),
                                    editor_length,
                                );
                                let prefix_padded = prefix.and_then(|s| {
                                    if s.is_empty() {
                                        None
                                    } else {
                                        Some(format!("{} ", s))
                                    }
                                });
                                Some(ProcValues {
                                    value: editor.format(options.global, locale),
                                    prefix: prefix_padded,
                                    suffix: None,
                                })
                            }
                            _ => {
                                let suffix = role_to_string(
                                    &self.contributor,
                                    locale,
                                    form.clone(),
                                    editor_length,
                                );
                                let suffix_padded = suffix.and_then(|s| {
                                    if s.is_empty() {
                                        None
                                    } else {
                                        Some(format!(" {}", s))
                                    }
                                });
                                Some(ProcValues {
                                    value: editor.format(options.global, locale),
                                    prefix: None,
                                    suffix: suffix_padded, // TODO handle None
                                })
                            }
                        }
                    }
                }
            }
            ContributorRole::Translator => Some(ProcValues {
                value: reference.translator()?.format(options.global, locale),
                prefix: None,
                suffix: None,
            }),
            ContributorRole::Publisher => Some(ProcValues {
                value: reference.publisher()?.format(options.global, locale),
                prefix: None,
                suffix: None,
            }),
            // TODO implement the rest
            _ => None,
        }
    }
}

impl ComponentValues for TemplateDate {
    fn values(
        &self,
        reference: &InputReference,
        hints: &ProcHints,
        options: &RenderOptions<'_>,
    ) -> Option<ProcValues> {
        let locale: &Locale = options.locale;
        let input_date: EdtfString = match &self.date {
            Dates::Issued => reference.issued()?,
            Dates::OriginalPublished => todo!("original-published"),
            Dates::Accessed => todo!("accessed"),
        };
        let parsed_date = input_date.parse();

        let formatted_date: String = match self.form {
            DateForm::Year => parsed_date
                .year() // this line causes a panic if the date is not a year
                .to_string(),
            DateForm::YearMonth => {
                input_date.year_month(locale.dates.months.long.clone())
            }
            DateForm::MonthDay => input_date.month_day(locale.dates.months.long.clone()),
            DateForm::Full => todo!(),
        };

        // TODO: implement this along with localized dates
        // TODO: implement this along with localized dates
        fn _config_fmt(options: &RenderOptions<'_>) -> DateTimeFormatterOptions {
            let date_options = match options.global.dates.clone() {
                Some(dates) => dates,
                None => return DateTimeFormatterOptions::default(), // or handle the None case accordingly
            };
            match date_options.month {
                MonthFormat::Long => todo!("long"),
                MonthFormat::Short => todo!("short"),
                MonthFormat::Numeric => todo!("numeric"),
            };
        }

        fn int_to_letter(n: u32) -> String {
            let c = n + 96;
            match char::from_u32(c) {
                Some(ch) => ch.to_string(),
                None => "".to_string(),
            }
        }

        let suffix = if hints.disamb_condition
            // TODO need to check form here also
            // && self.form == style::template::DateForm::Year
            // REVIEW: ugly, and needs to be smarter
            && options.global.processing.clone().unwrap_or_default().config().disambiguate.unwrap_or_default().year_suffix
            && formatted_date.len() == 4
        {
            int_to_letter((hints.group_index % 26) as u32)
        } else {
            "".to_string()
        };
        Some(ProcValues {
            value: formatted_date,
            prefix: None,
            suffix: Some(suffix), // put the suffix here, in case we need to do something with it
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::RenderOptions;
    use csln::bibliography::reference::{
        EdtfString, InputReference, Monograph, MonographType, NumOrStr, Serial,
        SerialComponent, SerialComponentType, SerialType, Title,
    };
    use csln::style::options::Config;

    fn mock_monograph() -> Monograph {
        Monograph {
            id: None,
            r#type: MonographType::Book,
            title: Title::Single("Title".to_string()),
            author: None,
            issued: EdtfString("2023".to_string()),
            publisher: None,
            url: None,
            accessed: None,
            note: None,
            isbn: None,
            doi: None,
            edition: None,
            translator: None,
        }
    }

    fn mock_serial_component() -> SerialComponent {
        SerialComponent {
            id: None,
            r#type: SerialComponentType::Article,
            title: Some(Title::Single("Article".to_string())),
            author: None,
            issued: EdtfString("2023".to_string()),
            parent: Serial {
                r#type: SerialType::AcademicJournal,
                title: Title::Single("Journal".to_string()),
            },
            url: None,
            accessed: None,
            note: None,
            doi: None,
            pages: None,
            volume: None,
            issue: None,
            translator: None,
        }
    }

    #[test]
    fn test_simple_string_values() {
        let config = Config::default();
        let locale = Locale::default();
        let options = RenderOptions { global: &config, local: &config, locale: &locale };
        let hints = ProcHints::default();

        let template_doi =
            TemplateSimpleString { variable: Variables::Doi, rendering: None };
        let mut serial = mock_serial_component();
        serial.doi = Some("10.1234/5678".to_string());
        let ref_doi = InputReference::SerialComponent(serial);
        let values = template_doi.values(&ref_doi, &hints, &options).unwrap();
        assert_eq!(values.value, "10.1234/5678");

        let template_isbn =
            TemplateSimpleString { variable: Variables::Isbn, rendering: None };
        let mut monograph = mock_monograph();
        monograph.isbn = Some("978-3-16-148410-0".to_string());
        let ref_isbn = InputReference::Monograph(monograph);
        let values = template_isbn.values(&ref_isbn, &hints, &options).unwrap();
        assert_eq!(values.value, "978-3-16-148410-0");
    }

    #[test]
    fn test_number_values() {
        let config = Config::default();
        let locale = Locale::default();
        let options = RenderOptions { global: &config, local: &config, locale: &locale };
        let hints = ProcHints::default();

        let template_vol = TemplateNumber {
            number: Numbers::Volume,
            form: None,
            rendering: None,
        };
        let mut serial = mock_serial_component();
        serial.volume = Some(NumOrStr::Number(42));
        let ref_vol = InputReference::SerialComponent(serial);
        let values = template_vol.values(&ref_vol, &hints, &options).unwrap();
        assert_eq!(values.value, "42");
    }

    #[test]
    fn test_title_values() {
        let config = Config::default();
        let locale = Locale::default();
        let options = RenderOptions { global: &config, local: &config, locale: &locale };
        let hints = ProcHints::default();

        let template_primary = TemplateTitle {
            title: Titles::Primary,
            form: None,
            rendering: None,
        };
        let monograph = mock_monograph();
        let ref_mono = InputReference::Monograph(monograph);
        let values = template_primary.values(&ref_mono, &hints, &options).unwrap();
        assert_eq!(values.value, "Title");
    }

    #[test]
    fn test_contributor_values() {
        let config = Config::default();
        let locale = Locale::default();
        let options = RenderOptions { global: &config, local: &config, locale: &locale };
        let hints = ProcHints::default();
        use csln::bibliography::reference::{Contributor, SimpleName};

        let template_author = TemplateContributor {
            contributor: ContributorRole::Author,
            form: ContributorForm::Long,
            rendering: None,
        };
        let mut monograph = mock_monograph();
        monograph.author = Some(Contributor::SimpleName(SimpleName {
            name: "John Smith".to_string(),
            location: None,
        }));
        let ref_mono = InputReference::Monograph(monograph);
        let values = template_author.values(&ref_mono, &hints, &options).unwrap();
        assert_eq!(values.value, "John Smith");
    }

    #[test]
    fn test_date_disambiguation() {
        use csln::style::options::{Disambiguation, Processing, ProcessingCustom};

        let mut config_inner = Config::default();
        config_inner.processing = Some(Processing::Custom(ProcessingCustom {
            disambiguate: Some(Disambiguation {
                year_suffix: true,
                ..Default::default()
            }),
            ..Default::default()
        }));

        let locale = Locale::default();
        let options = RenderOptions {
            global: &config_inner,
            local: &config_inner,
            locale: &locale,
        };

        let mut hints = ProcHints::default();
        hints.disamb_condition = true;
        hints.group_index = 1; // 'a'

        let template_date = TemplateDate {
            date: Dates::Issued,
            form: DateForm::Year,
            rendering: None,
        };
        let monograph = mock_monograph();
        let ref_mono = InputReference::Monograph(monograph);

        let values = template_date.values(&ref_mono, &hints, &options).unwrap();
        assert_eq!(values.value, "2023");
        assert_eq!(values.suffix, Some("a".to_string()));

        hints.group_index = 2; // 'b'
        let values = template_date.values(&ref_mono, &hints, &options).unwrap();
        assert_eq!(values.suffix, Some("b".to_string()));
    }
}
