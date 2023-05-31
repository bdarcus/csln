#[cfg(test)]
mod tests {
    use csln_processor::{load_style_from_file, load_bibliography_from_file};

    // create tests for Processor::get_proc_references and Processor::sort_proc_references
    #[test]
    fn test_get_proc_references() {
        let style = load_style_from_file("examples/style.csl.yaml");
        let bibliography = load_bibliography_from_file("examples/bibliography.yaml");
        let processor = csln_processor::Processor::new(style, bibliography, "en-US".to_string());
        let proc_references = processor.get_proc_references();
        assert_eq!(proc_references.len(), 5);
        // how can I test the contents of proc_references?
        assert_eq!(proc_references[0].data.title.as_deref(), Some("A Title"));
        assert_eq!(proc_references[4].data.title.as_deref(), Some("B Title"));
    }
}
