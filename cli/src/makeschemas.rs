use schemars::schema_for;
use std::fs;
use std::fs::File;
use std::io::Write;

use style::Style;
use citation::Citation;
use bibliography::InputBibliography;

fn main() {
    fs::create_dir_all("schemas").unwrap();

    let style_schema = schema_for!(Style);
    let citation_schema = schema_for!(Citation);
    let bib_schema = schema_for!(InputBibliography);

    let style_json_output = serde_json::to_string_pretty(&style_schema).unwrap();
    let citation_json_output = serde_json::to_string_pretty(&citation_schema).unwrap();
    let bib_json_output = serde_json::to_string_pretty(&bib_schema).unwrap();

    let mut citation_file = File::create("schemas/citation.json").unwrap();
    let mut style_file = File::create("schemas/style.json").unwrap();
    let mut bib_file = File::create("schemas/bibliography.json").unwrap();
    style_file.write_all(style_json_output.as_bytes()).unwrap();
    citation_file.write_all(citation_json_output.as_bytes()).unwrap();
    bib_file.write_all(bib_json_output.as_bytes()).unwrap();
    println!("Wrote bibliography schema to schemas/bibliography.json");
    println!("Wrote citation schema to schemas/citation.json");
    println!("Wrote style schema to schemas/style.json");
}
