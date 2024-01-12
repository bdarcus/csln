#[cfg(test)]
mod tests {
    use csln::citation::{Citation, CitationItem, Citations};
    use csln::from_file;

    #[allow(dead_code)]
    // FIXME why these warnings?
    struct TestFixture {
        style: csln::style::Style,
        locale: csln::style::locale::Locale,
        bibliography: csln::bibliography::InputBibliography,
        citations: Vec<Citation>,
        processor: csln_processor::Processor,
    }

    fn setup() -> TestFixture {
        let style = from_file("examples/style.csl.yaml");
        let locale = from_file("locales/locale-en.yaml");
        let bibliography = from_file("examples/ex1.bib.yaml");
        let citations: Citations =
            from_file("examples/citation.yaml").context("Citation file?");
        let processor =
            csln_processor::Processor::new(style, bibliography, citations, locale);

        TestFixture { style, locale, bibliography, citations, processor }
    }

    #[test]
    fn gets_references() {
        let fixture = setup();
        assert_eq!(fixture.processor.get_references().len(), 36);
        assert!(fixture.processor.get_reference("doe1").is_ok());
        assert_eq!(
            fixture.processor.get_reference("doe1").unwrap().title(),
            Some(csln::bibliography::reference::Title::Single("Title 2".to_string()))
        );
        assert!(fixture.processor.get_proc_hints().contains_key("doe1"));
    }

    #[test]
    fn sorts_references() {
        let fixture = setup();
        let refs = fixture.processor.get_references();
        let sorted_refs = fixture.processor.sort_references(refs);
        assert_eq!(sorted_refs.len(), 36);
        assert_eq!(sorted_refs.last().unwrap().title().unwrap().to_string(), "Title 4");
    }

    #[test]
    fn process_citation_item() {
        // TODO make it for citations as a whole, and confirm no empty ones
        let fixture = setup();
        let citation_item = CitationItem {
            ref_id: "doe1".to_string(),
            label: None,
            prefix: Some("Prefix".to_string()),
            suffix: None,
        };
        let result = fixture.processor.process_citation_item(&citation_item);
        // confirm
        // assert_eq!(fixture.processor.get_reference("doe1"), "doe1".to_string());
        assert_eq!(result.unwrap()[0].values.value.to_string(), "Doe, Jane".to_string());
    }

    #[test]
    fn derives_proc_hints() {
        let fixture = setup();
        let proc_hints = fixture.processor.get_proc_hints();
        assert_eq!(proc_hints["doe7"].group_index, 1);
        assert_eq!(proc_hints["doe7"].group_length, 1);
    }

    #[test]
    fn loads_and_parses_locale_file() {
        let fixture = setup();
        assert_eq!(fixture.locale.dates.months.long[0], "January");
        assert_eq!(fixture.locale.dates.months.long[11], "December");
        assert_eq!(fixture.locale.dates.months.short[0], "Jan");
        assert_eq!(fixture.locale.dates.months.short[11], "Dec");
    }
}
