/*
SPDX-License-Identifier: MPL-2.0
SPDX-FileCopyrightText: © 2023-2026 Bruce D'Arcus
*/

use schemars::schema_for;
use std::fs;
use std::fs::File;
use std::io::Write;

use csln::bibliography::InputBibliography;
use csln::citation::CitationList;
use csln::style::locale::Locale;
use csln::style::Style;

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    fs::create_dir_all("schemas").context("Failed to create directory 'schemas'")?;

    let style_schema = schema_for!(Style);
    let citation_schema = schema_for!(CitationList);
    let bib_schema = schema_for!(InputBibliography);
    let locale_schema = schema_for!(Locale);

    let style_json_output = serde_json::to_string_pretty(&style_schema)
        .context("Failed to serialize style schema")?;
    let citation_json_output = serde_json::to_string_pretty(&citation_schema)
        .context("Failed to serialize citation schema")?;
    let bib_json_output = serde_json::to_string_pretty(&bib_schema)
        .context("Failed to serialize bibliography schema")?;
    let locale_json_output = serde_json::to_string_pretty(&locale_schema)
        .context("Failed to serialize locale schema")?;

    let mut citation_file = File::create("schemas/citation.json")
        .context("Failed to create schemas/citation.json")?;
    let mut style_file = File::create("schemas/style.json")
        .context("Failed to create schemas/style.json")?;
    let mut bib_file = File::create("schemas/bibliography.json")
        .context("Failed to create schemas/bibliography.json")?;
    let mut locale_file = File::create("schemas/locale.json")
        .context("Failed to create schemas/locale.json")?;

    style_file
        .write_all(style_json_output.as_bytes())
        .context("Failed to write style schema")?;
    citation_file
        .write_all(citation_json_output.as_bytes())
        .context("Failed to write citation schema")?;
    bib_file
        .write_all(bib_json_output.as_bytes())
        .context("Failed to write bibliography schema")?;
    locale_file
        .write_all(locale_json_output.as_bytes())
        .context("Failed to write locale schema")?;

    println!("Wrote bibliography schema to schemas/bibliography.json");
    println!("Wrote citation schema to schemas/citation.json");
    println!("Wrote style schema to schemas/style.json");
    println!("Wrote locale schema to schemas/locale.json");

    Ok(())
}
