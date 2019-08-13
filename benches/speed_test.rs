#[macro_use]
extern crate criterion;

use criterion::Criterion;
use gml_fmt;
use gml_fmt::config::{Config, PrintFlags};
use std::{path::PathBuf, process};

fn lex_test() {
    let path = PathBuf::from("benches/samples/osg_lex_speed.gml");
    let config = Config::new(path, PrintFlags::NO_OUTPUT, true).unwrap_or_else(|e| {
        eprintln!("File reading error: {}", e);
        process::exit(1);
    });

    gml_fmt::run_config(&config, &gml_fmt::lang_config::LangConfig::default())
        .expect("Attempted to run osg_lex_speed test, but failed. Did you move the file?");
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("osg_lex_speed", |b| b.iter(|| lex_test()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
