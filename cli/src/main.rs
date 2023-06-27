use bibliography::HasFile;
use bibliography::InputBibliography as Bibliography;
use citation::Citation;
use clap::Parser;
use processor::Processor;
use style::Style;

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
    let locale = style::locale::Locale::from_file(&opts.locale);
    let processor: Processor = Processor::new(style, bibliography, citations, locale);
    let rendered_refs = processor.render_references();
    println!("{}", serde_json::to_string_pretty(&rendered_refs).unwrap());
}
