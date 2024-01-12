use anyhow::Context;
use clap::Parser;
use csln::citation::Citations;
use csln::from_file;
use processor::{ProcReferences, Processor};

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
    citations: Option<String>,
    #[clap(short, long)]
    /// The path to the CSLN locale file
    locale: String,
}

fn main() {
    let opts = Opts::parse();
    let style = from_file(&opts.style).context("Style file?");
    let bibliography = from_file(&opts.bibliography).context("Bibliography file?");
    let citations: Citations = if opts.citations.is_none() {
        Citations::default()
    } else {
        from_file(opts.citations.unwrap()).unwrap_or_default()
    };
    let locale = from_file(&opts.locale).context("Locale file?");
    let processor: Processor = Processor::new(
        style.expect("msg"), // REVIEW why?
        bibliography.expect("msg"),
        citations,
        locale.expect("msg"),
    );
    let rendered_refs: ProcReferences = processor.process_references();
    let serialized_refs = serde_json::to_string_pretty(&rendered_refs);
    //println!("{}", refs_to_string(rendered_refs));
    if serialized_refs.is_err() {
        println!("Error: {:?}", serialized_refs);
    } else {
        println!("{}", serialized_refs.unwrap());
    }
}
