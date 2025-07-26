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

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    let style = from_file(&opts.style).context("Failed to load style file")?;
    let bibliography = from_file(&opts.bibliography).context("Failed to load bibliography file")?;
    let citations: Citations = if let Some(citation_path) = opts.citations {
        from_file(&citation_path).context("Failed to load citation file")?
    } else {
        Citations::default()
    };
    let locale = from_file(&opts.locale).context("Failed to load locale file")?;
    let processor: Processor = Processor::new(style, bibliography, citations, locale);
    let rendered_refs: ProcReferences = processor.process_references();
    let serialized_refs = serde_json::to_string_pretty(&rendered_refs)
        .context("Failed to serialize references")?;
    println!("{}", serialized_refs);
    Ok(())
}
