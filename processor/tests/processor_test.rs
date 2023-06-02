#[cfg(test)]
mod tests {
    use csln_processor::{load_bibliography_from_file, load_style_from_file};

    // create tests for Processor::get_proc_references and Processor::sort_proc_references
    #[test]
    fn test_get_proc_references() {
        let style = load_style_from_file("examples/style.csl.yaml");
        let bibliography = load_bibliography_from_file("examples/ex1.bib.yaml");
        let processor = csln_processor::Processor::new(style, bibliography, "en-US".to_string());
        let proc_refs = processor.get_proc_references();
        assert_eq!(proc_refs.len(), 5);
        // how can I test the contents of proc_references?
        assert_eq!(proc_refs[0].data.title.as_deref(), Some("Title 0"));
        assert_eq!(proc_refs[4].data.title.as_deref(), Some("Title 4"));
    }

    #[test]
    fn test_get_proc_hints() {
        let style = load_style_from_file("examples/style.csl.yaml");
        let bibliography = load_bibliography_from_file("examples/ex1.bib.yaml");
        let processor = csln_processor::Processor::new(style, bibliography, "en-US".to_string());
        let proc_refs = processor.get_proc_references();
        assert_eq!(proc_refs.len(), 20);
        assert_eq!(proc_refs[0].proc_hints.group_index, 1);
        assert_eq!(proc_refs[1].proc_hints.group_index, 2);
    }
}
