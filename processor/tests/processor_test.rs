#[test]
fn test_group_proc_references() {
    let processor = Processor::new(Bibliography::default(), Style::default());
    let proc_references = processor.get_proc_references();
    let grouped_proc_references = processor.group_proc_references(proc_references);
    assert_eq!(grouped_proc_references.len(), 4);
    assert_eq!(
        grouped_proc_references[0].proc_hints.as_ref().unwrap().group_key,
        "Doe2020"
    );
    assert_eq!(
        grouped_proc_references[0].proc_hints.as_ref().unwrap().group_index,
        0
    );
    assert_eq!(
        grouped_proc_references[0].proc_hints.as_ref().unwrap().group_length,
        2
    );
    assert_eq!(
        grouped_proc_references[0].proc_hints.as_ref().unwrap().disamb_condition,
        false
    );
    assert_eq!(
        grouped_proc_references[1].proc_hints.as_ref().unwrap().group_key,
        "Doe2021"
    );
    assert_eq!(
        grouped_proc_references[1].proc_hints.as_ref().unwrap().group_index,
        1
    );
    assert_eq!(
        grouped_proc_references[1].proc_hints.as_ref().unwrap().group_length,
        1
    );
    assert_eq!(
        grouped_proc_references[1].proc_hints.as_ref().unwrap().disamb_condition,
        false
    );
    assert_eq!(
        grouped_proc_references[2].proc_hints.as_ref().unwrap().group_key,
        "Smith2020"
    );
    assert_eq!(
        grouped_proc_references[2].proc_hints.as_ref().unwrap().group_index,
        2
    );
    assert_eq!(
        grouped_proc_references[2].proc_hints.as_ref().unwrap().group_length,
        1
    );
    assert_eq!(
        grouped_proc_references[2].proc_hints.as_ref().unwrap().disamb_condition,
        false
    );
    assert_eq!(
        grouped_proc_references[3].proc_hints.as_ref().unwrap().group_key,
        "Smith2021"
    );
    assert_eq!(
        grouped_proc_references[3].proc_hints.as_ref().unwrap().group_index,
        3
    );
    assert_eq!(
        grouped_proc_references[3].proc_hints.as_ref().unwrap().group_length,
        1
    );
    assert_eq!(
        grouped_proc_references[3].proc_hints.as_ref().unwrap().disamb_condition,
        false
    );
}#[cfg(test)]
mod tests {
    use csln_processor::{load_style_from_file, load_bibliography_from_file};

    // create tests for Processor::get_proc_references and Processor::sort_proc_references
    #[test]
    fn test_get_proc_references() {
        let style = load_style_from_file("examples/style.csl.yaml");
        let bibliography = load_bibliography_from_file("examples/ex1.bib.yaml");
        let processor = csln_processor::Processor::new(style, bibliography, "en-US".to_string());
        let proc_references = processor.get_proc_references();
        assert_eq!(proc_references.len(), 5);
        // how can I test the contents of proc_references?
        assert_eq!(proc_references[0].data.title.as_deref(), Some("A Title"));
        assert_eq!(proc_references[4].data.title.as_deref(), Some("B Title"));
    }
}
