use bibliography::InputBibliography as Bibliography;
use processor::Processor;
use std::env;
use style::Style;

use processor::{load_bibliography_from_file, load_style_from_file};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Please provide the path of the style and bibliography files as command line arguments.");
    }

    let style_path: &String = &args[1];
    let style: Style = load_style_from_file(style_path);

    let bibliography_path: &String = &args[2];
    let bibliography: Bibliography = load_bibliography_from_file(bibliography_path);
    let processor: Processor = Processor::new(style, bibliography, bibliography_path.to_string());
    println!("{:?}", processor.get_proc_references());
}
