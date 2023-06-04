use bibliography::{InputBibliography as Bibliography};
use criterion::{criterion_group, criterion_main, Criterion};
use csln_processor::Processor;
use csln_processor::{load_bibliography_from_file, load_style_from_file};
use style::Style;

fn proc_benchmark(c: &mut Criterion) {
    let style: Style = load_style_from_file("examples/style.csl.yaml");
    let bibliography: Bibliography = load_bibliography_from_file("examples/ex1.bib.yaml");
    let processor: Processor = Processor::new(style, bibliography, "en-US".to_string());
    c.bench_function("sorting references", |b| {
        b.iter(|| {
            let refs = processor.get_references();
            processor.sort_references(refs);
        })
    });
    c.bench_function("rendering references", |b| {
        b.iter(|| {
            processor.render_references();
        })
    });
}

criterion_group!(benches, proc_benchmark);
criterion_main!(benches);
