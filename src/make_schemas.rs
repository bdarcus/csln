use schemars::schema_for;
use std::fs;
use std::fs::File;
use std::io::Write;

pub mod bibliography;
mod style;
use style::Style;

use crate::bibliography::InputBibliography;

fn main() {
    fs::create_dir_all("schemas").unwrap();

    let style_schema = schema_for!(Style);
    let bib_schema = schema_for!(InputBibliography);

    let style_json_output = serde_json::to_string_pretty(&style_schema).unwrap();
    let bib_json_output = serde_json::to_string_pretty(&bib_schema).unwrap();

    let mut style_file = File::create("schemas/style.json").unwrap();
    let mut bib_file = File::create("schemas/bibliography.json").unwrap();
    style_file.write_all(style_json_output.as_bytes()).unwrap();
    bib_file.write_all(bib_json_output.as_bytes()).unwrap();
    println!("Wrote style schema to schemas/style.json");
    println!(
        "Wrote bibliography schema to schemas/bibliography.json"
    );
}
