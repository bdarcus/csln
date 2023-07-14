#[cfg(test)]
mod tests {
    use csln::bibliography::HasFile;
    use csln::citation::Citation;
    // create tests for Processor::get_proc_references and Processor::sort_proc_references
    #[test]
    fn sorts_references() {
        let style = csln::style::Style::from_file("examples/style.csl.yaml");
        let locale = csln::style::locale::Locale::from_file("locales/locale-en.yaml");
        let bibliography =
            csln::bibliography::InputBibliography::from_file("examples/ex1.bib.yaml");
        let citations: Vec<Citation> = Vec::new();
        let processor =
            csln_processor::Processor::new(style, bibliography, citations, locale);
        let refs = processor.get_references();
        let sorted_refs = processor.sort_references(refs);
        assert_eq!(sorted_refs.len(), 36);
        assert_eq!(sorted_refs.last().unwrap().title().unwrap().to_string(), "Title 4");
    }

    #[test]
    fn derives_proc_hints() {
        let style = csln::style::Style::from_file("examples/style.csl.yaml");
        let locale = csln::style::locale::Locale::from_file("locales/locale-en.yaml");
        let citations: Vec<Citation> = Vec::new();
        let bibliography =
            csln::bibliography::InputBibliography::from_file("examples/ex1.bib.yaml");
        let processor =
            csln_processor::Processor::new(style, bibliography, citations, locale);
        let proc_hints = processor.get_proc_hints();
        assert_eq!(proc_hints["doe7"].group_index, 1);
        assert_eq!(proc_hints["doe7"].group_length, 1);
    }

    #[test]
    fn loads_and_parses_locale_file() {
        let locale = csln::style::locale::Locale::from_file("locales/locale-en.yaml");
        assert_eq!(locale.dates.months.long[0], "January");
        assert_eq!(locale.dates.months.long[11], "December");
        assert_eq!(locale.dates.months.short[0], "Jan");
        assert_eq!(locale.dates.months.short[11], "Dec");
    }
}
