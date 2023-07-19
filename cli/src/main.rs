use clap::Parser;
use csln::bibliography::HasFile;
use csln::bibliography::InputBibliography as Bibliography;
use csln::citation::Citation;
use csln::style::Style;
use processor::{refs_to_string, Processor};

#[derive(Parser, Default, Debug)]
#[clap(author = "Bruce D'Arcus", version, about = "A CLI for CSLN")]
pub struct Opts {
    #[clap(short, long)]
    /// The path to the CSLN style file
    style: String,
    #[clap(short, long)]
    /// The path to the CSLN bibliography file
    bibliography: String,
    #[clap(short, long)]
    /// The path to the CSLN locale file
    locale: String,
}

fn main() {
    let opts = Opts::parse();
    let style: Style = Style::from_file(&opts.style);
    let bibliography: Bibliography = Bibliography::from_file(&opts.bibliography);
    let citations: Vec<Citation> = Vec::new();
    let locale = csln::style::locale::Locale::from_file(&opts.locale);
    let processor: Processor = Processor::new(style, bibliography, citations, locale);
    let rendered_refs = processor.render_references();
    println!("{}", refs_to_string(rendered_refs));
    //println!("{}", serde_json::to_string_pretty(&rendered_refs).unwrap());
}
