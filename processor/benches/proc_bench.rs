#![allow(clippy::expect_used)]
use criterion::{criterion_group, criterion_main, Criterion};
use csln::bibliography::InputBibliography as Bibliography;
use csln::citation::Citation;
use csln::from_file;
use csln_processor::Processor;
use std::time::Duration;

fn proc_benchmark(c: &mut Criterion) {
    let style = match from_file("examples/style.csl.yaml") {
        Ok(style) => style,
        Err(_) => {
            println!("Failed to load style");
            return;
        }
    };
    let bibliography: Bibliography = from_file("examples/ex1.bib.yaml").expect("msg");
    let locale = from_file("locales/locale-en.yaml");
    let citations: Vec<Citation> = Vec::new();
    let processor: Processor =
        Processor::new(style, bibliography, citations, locale.expect("msg"));
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
            processor.process_references();
        })
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::new(12, 0)).sample_size(80);
    targets = proc_benchmark
);
criterion_main!(benches);
