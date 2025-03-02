use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

// I used Claude 3.7 as the starting point for this code

use citationberg::csl::{self, Style as CslStyle};

// Import CSLN types from your crate
// Replace "csln" with the actual name of your crate if different
use csln::style::{
    Bibliography, Citation, CitationOptions, DateElement, DisambiguateOptions,
    FormattingAttributes, GroupElement, Metadata, NameFormat, NamesElement,
    RenderingElement, SortKey, Style, TextElement,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Example usage
    let xml_file = "path/to/your/csl/style.csl";
    let output_json_file = "output.json";
    let output_yaml_file = "output.yaml";

    let csln_style = convert_csl_to_csln(xml_file)?;

    // Save as JSON
    let json_string = serde_json::to_string_pretty(&csln_style)?;
    fs::write(output_json_file, json_string)?;

    // Save as YAML
    let yaml_string = serde_yaml::to_string(&csln_style)?;
    fs::write(output_yaml_file, yaml_string)?;

    println!("Conversion completed successfully!");
    Ok(())
}

fn convert_csl_to_csln(csl_path: &str) -> Result<Style, Box<dyn Error>> {
    let csl_content = fs::read_to_string(csl_path)?;
    let style = citationberg::Citationberg::from_xml_str(&csl_content)?;

    match style {
        citationberg::Citationberg::Style(style) => convert_style_to_csln(style),
        citationberg::Citationberg::Locale(_) => {
            Err("Cannot convert locale files to CSLN".into())
        }
    }
}

fn convert_style_to_csln(style: CslStyle) -> Result<Style, Box<dyn Error>> {
    // Create the CSLN Style object
    let mut csln_style = Style {
        metadata: Metadata {
            id: style.info.id,
            title: style.info.title,
            version: "1.0.0".to_string(), // Default version for CSLN
            description: style.info.description,
            authors: None, // Could be extracted from CSL if needed
        },
        citation: None,
        bibliography: None,
        macros: HashMap::new(),
    };

    // Convert citation specification
    if let Some(citation) = style.citation {
        csln_style.citation = Some(convert_citation_to_csln(&citation)?);
    }

    // Convert bibliography specification
    if let Some(bibliography) = style.bibliography {
        csln_style.bibliography = Some(convert_bibliography_to_csln(&bibliography)?);
    }

    // Convert macros
    csln_style.macros = convert_macros_to_csln(&style.macro_map)?;

    Ok(csln_style)
}

fn convert_citation_to_csln(
    citation: &csl::Citation,
) -> Result<Citation, Box<dyn Error>> {
    let mut csln_citation = Citation {
        layout: Vec::new(),
        sort: citation.sort.unwrap_or(false),
        options: None,
    };

    // Convert layout
    if let Some(layout) = &citation.layout {
        csln_citation.layout = convert_layout_to_csln(layout)?;
    }

    // Convert options
    if let Some(options) = &citation.options {
        csln_citation.options = Some(CitationOptions {
            collapse: options.collapse.clone(),
            disambiguate: if options.disambiguate_add_names
                || options.disambiguate_add_year_suffix
            {
                Some(DisambiguateOptions {
                    add_names: options.disambiguate_add_names,
                    add_year_suffix: options.disambiguate_add_year_suffix,
                })
            } else {
                None
            },
        });
    }

    Ok(csln_citation)
}

fn convert_bibliography_to_csln(
    bibliography: &csl::Bibliography,
) -> Result<Bibliography, Box<dyn Error>> {
    let mut csln_bibliography = Bibliography { layout: Vec::new(), sort: None };

    // Convert layout
    if let Some(layout) = &bibliography.layout {
        csln_bibliography.layout = convert_layout_to_csln(layout)?;
    }

    // Convert sorting rules if present
    if let Some(sort) = &bibliography.sort {
        let mut sort_keys = Vec::new();
        for key in &sort.keys {
            sort_keys.push(SortKey {
                variable: key.variable.clone(),
                macro_name: key.macro_name.clone(),
                sort: key
                    .sort
                    .as_ref()
                    .map_or_else(|| "ascending".to_string(), |s| s.to_string()),
            });
        }
        csln_bibliography.sort = Some(sort_keys);
    }

    Ok(csln_bibliography)
}

fn convert_layout_to_csln(
    layout: &csl::Layout,
) -> Result<Vec<RenderingElement>, Box<dyn Error>> {
    let mut elements = Vec::new();

    for rendering_element in &layout.rendering_elements {
        elements.push(convert_rendering_element_to_csln(rendering_element)?);
    }

    Ok(elements)
}

fn convert_rendering_element_to_csln(
    element: &csl::RenderingElement,
) -> Result<RenderingElement, Box<dyn Error>> {
    match element {
        csl::RenderingElement::Text(text) => {
            let text_element = TextElement {
                value: text.value.clone(),
                variable: text.variable.as_ref().map(|v| v.to_string()),
                macro_name: text.macro_name.clone(),
                formatting: extract_formatting_attributes(text),
            };

            Ok(RenderingElement::Text(text_element))
        }
        csl::RenderingElement::Date(date) => {
            let date_element = DateElement {
                variable: date.variable.to_string(),
                parts: date.date_parts.clone(),
                formatting: extract_formatting_attributes(date),
            };

            Ok(RenderingElement::Date(date_element))
        }
        csl::RenderingElement::Names(names) => {
            // Extract variables as a comma-separated string
            let variable = names.variables.join(", ");

            let name_format = if let Some(name) = &names.name {
                Some(NameFormat {
                    form: name.form.as_ref().map(|f| f.to_string()),
                    initialize: name.initialize,
                    initialize_with: name.initialize_with.clone(),
                    delimiter: name.delimiter.clone(),
                })
            } else {
                None
            };

            let names_element = NamesElement {
                variable,
                format: name_format,
                formatting: extract_formatting_attributes(names),
            };

            Ok(RenderingElement::Names(names_element))
        }
        csl::RenderingElement::Group(group) => {
            let mut items = Vec::new();
            for child in &group.rendering_elements {
                items.push(convert_rendering_element_to_csln(child)?);
            }

            let group_element = GroupElement {
                delimiter: group.delimiter.clone().unwrap_or_default(),
                items,
                formatting: extract_formatting_attributes(group),
            };

            Ok(RenderingElement::Group(group_element))
        }
        // Handle other element types as needed
        _ => {
            // This is a placeholder. You would implement conversion for other elements
            // based on your CSLN schema
            Err("Unsupported rendering element type".into())
        }
    }
}

fn extract_formatting_attributes<T: csl::Formatting>(
    element: &T,
) -> FormattingAttributes {
    FormattingAttributes {
        font_style: element.font_style().map(|s| s.to_string()),
        font_weight: element.font_weight().map(|w| w.to_string()),
        text_decoration: element.text_decoration().map(|d| d.to_string()),
        text_case: element.text_case().map(|c| c.to_string()),
    }
}

fn convert_macros_to_csln(
    macro_map: &HashMap<String, csl::Macro>,
) -> Result<HashMap<String, Vec<RenderingElement>>, Box<dyn Error>> {
    let mut csln_macros = HashMap::new();

    for (name, macro_def) in macro_map {
        let mut elements = Vec::new();

        for element in &macro_def.rendering_elements {
            elements.push(convert_rendering_element_to_csln(element)?);
        }

        csln_macros.insert(name.clone(), elements);
    }

    Ok(csln_macros)
}

// Function to batch convert a directory of CSL files
pub fn convert_directory(
    input_dir: &Path,
    output_dir: &Path,
    format: &str,
) -> Result<(), Box<dyn Error>> {
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
    }

    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "csl") {
            let file_stem = path.file_stem().unwrap().to_string_lossy();
            let csln_style = convert_csl_to_csln(path.to_str().unwrap())?;

            let output_path = match format {
                "json" => output_dir.join(format!("{}.json", file_stem)),
                "yaml" => output_dir.join(format!("{}.yaml", file_stem)),
                _ => return Err(format!("Unsupported output format: {}", format).into()),
            };

            match format {
                "json" => {
                    fs::write(&output_path, serde_json::to_string_pretty(&csln_style)?)?
                }
                "yaml" => fs::write(&output_path, serde_yaml::to_string(&csln_style)?)?,
                _ => unreachable!(),
            }

            println!("Converted: {} -> {}", path.display(), output_path.display());
        }
    }

    Ok(())
}

// Function to work directly with the CSLN model
// This will be useful if you want to add this to your library
pub fn convert_csl_string_to_csln_model(csl_xml: &str) -> Result<Style, Box<dyn Error>> {
    let style = citationberg::Citationberg::from_xml_str(csl_xml)?;

    match style {
        citationberg::Citationberg::Style(style) => convert_style_to_csln(style),
        citationberg::Citationberg::Locale(_) => {
            Err("Cannot convert locale files to CSLN".into())
        }
    }
}
