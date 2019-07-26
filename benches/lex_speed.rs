#[macro_use]
extern crate criterion;

use criterion::Criterion;
use gml_fmt;

fn lex_test() {
    gml_fmt::run_config_test_file_output("benches/samples/osg_lex_speed.gml")
        .expect("Attempted to run osg_lex_speed test, but failed. Did you move the file?");
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("osg_lex_speed", |b| b.iter(|| lex_test()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);