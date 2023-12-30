use clap::Parser;
use csln::bibliography::InputBibliography as Bibliography;
use csln::citation::Citations;
use csln::style::Style;
use csln::HasFile;
use processor::Processor;

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
    /// The optional path to the CSLN citation file
    citation: Option<String>,
    #[clap(short, long)]
    /// The path to the CSLN locale file
    locale: String,
}

fn main() {
    let opts = Opts::parse();
    let style: Style = Style::from_file(&opts.style);
    let bibliography: Bibliography = Bibliography::from_file(&opts.bibliography);
    let citations: Citations = if opts.citation.is_none() {
        Citations::default()
    } else {
        Citations::from_file(&opts.citation.unwrap_or_default())
    };
    let locale = csln::style::locale::Locale::from_file(&opts.locale);
    let processor: Processor = Processor::new(style, bibliography, citations, locale);
    let rendered_refs = processor.process_references();
    //println!("{}", refs_to_string(rendered_refs));
    println!("{}", serde_json::to_string_pretty(&rendered_refs).unwrap());
}
