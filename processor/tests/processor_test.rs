#[cfg(test)]
mod tests {
    use csln_processor::{load_bibliography_from_file, load_style_from_file};

    // create tests for Processor::get_proc_references and Processor::sort_proc_references
    #[test]
    fn test_get_proc_references() {
        let style = load_style_from_file("examples/style.csl.yaml");
        let bibliography = load_bibliography_from_file("examples/ex1.bib.yaml");
        let processor = csln_processor::Processor::new(style, bibliography, "en-US".to_string());
        let refs = processor.get_references();
        let sorted_refs = processor.sort_references(refs);
        assert_eq!(sorted_refs.len(), 14);
        // how can I test the contents of proc_references?
        assert_eq!(sorted_refs[0].title.as_deref(), Some("Title 0"));
        assert_eq!(sorted_refs[4].title.as_deref(), Some("Title 4"));
    }

    #[test]
    fn test_get_proc_hints() {
        let style = load_style_from_file("examples/style.csl.yaml");
        let bibliography = load_bibliography_from_file("examples/ex1.bib.yaml");
        let processor = csln_processor::Processor::new(style, bibliography, "en-US".to_string());
        let proc_hints = processor.get_proc_hints();
        assert_eq!(proc_hints.keys().len(), 14);
        assert_eq!(proc_hints["id-0"].group_index, 0);
        assert!(proc_hints["id-1"].disamb_condition, "false");
    }
}
