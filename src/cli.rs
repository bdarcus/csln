mod processor;
use processor::Processor;
mod style;
use style::Style;
mod bibliography;
use bibliography::InputBibliography as Bibliography;
use std::env;
use std::fs;

fn load_style_from_yaml(style_path: &str) -> Style {
    let contents = fs::read_to_string(style_path).expect("Failed to read style file");
    serde_yaml::from_str(&contents).expect("Failed to parse YAML")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Please provide the path of the style and bibliography files as command line arguments.");
    }

    let style_path = &args[1];
    let bibliography_path = &args[2];

    let style = load_style_from_yaml(style_path);
    let bibliography = Bibliography::new(bibliography_path.to_string());
    let processor = Processor::new(style, bibliography);
    println!("{:?}", processor);
}
