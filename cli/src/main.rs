use bibliography::InputBibliography as Bibliography;
use bibliography::HasFile;
use processor::Processor;
use std::env;
use style::Style;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Please provide the path of the style and bibliography files as command line arguments.");
    }

    let style_path: &String = &args[1];
    let style: Style = Style::from_file(style_path);

    let bibliography_path: &String = &args[2];
    let bibliography: Bibliography = Bibliography::from_file(bibliography_path);
    let processor: Processor = Processor::new(style, bibliography, bibliography_path.to_string());
    let rendered_refs = processor.render_references();
    println!("{}", serde_json::to_string_pretty(&rendered_refs).unwrap());
}
