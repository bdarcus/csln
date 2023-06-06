#[cfg(test)]
mod tests {
    use bibliography::HasFile;
    // create tests for Processor::get_proc_references and Processor::sort_proc_references
    #[test]
    fn test_sort_references() {
        let style = style::Style::from_file("examples/style.csl.yaml");
        let bibliography = bibliography::InputBibliography::from_file("examples/ex1.bib.yaml");
        let processor = csln_processor::Processor::new(style, bibliography, "en-US".to_string());
        let refs = processor.get_references();
        let sorted_refs = processor.sort_references(refs);
        assert_eq!(sorted_refs.len(), 36);
        assert_eq!(sorted_refs[35].title.as_deref(), Some("Title 4"));
        assert_eq!(sorted_refs[1].title.as_deref(), Some("Title 31"));
    }

    #[test]
    fn test_proc_hints() {
        let style = style::Style::from_file("examples/style.csl.yaml");
        let bibliography = bibliography::InputBibliography::from_file("examples/ex1.bib.yaml");
        let processor = csln_processor::Processor::new(style, bibliography, "en-US".to_string());
        let proc_hints = processor.get_proc_hints();
        assert_eq!(proc_hints["doe7"].group_index, 1);
        assert_eq!(proc_hints["doe7"].group_length, 2);
    }
}
