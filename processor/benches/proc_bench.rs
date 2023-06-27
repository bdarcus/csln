use bibliography::HasFile;
use bibliography::InputBibliography as Bibliography;
use citation::Citation;
use criterion::{criterion_group, criterion_main, Criterion};
use csln_processor::Processor;
use std::time::Duration;
use style::Style;

fn proc_benchmark(c: &mut Criterion) {
    let style: Style = style::Style::from_file("examples/style.csl.yaml");
    let bibliography: Bibliography =
        bibliography::InputBibliography::from_file("examples/ex1.bib.yaml");
    let locale = style::locale::Locale::from_file("locales/locale-en.yaml");
    let citations: Vec<Citation> = Vec::new();
    let processor: Processor = Processor::new(style, bibliography, citations, locale);
    c.bench_function("sorting references", |b| {
        b.iter(|| {
            let refs = processor.get_references();
            processor.sort_references(refs);
        })
    });
    c.bench_function("grouping references", |b| {
        b.iter(|| {
            processor.group_references(processor.get_references());
        })
    });
    c.bench_function("rendering references", |b| {
        b.iter(|| {
            processor.render_references();
        })
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::new(12, 0)).sample_size(80);
    targets = proc_benchmark
);
criterion_main!(benches);
