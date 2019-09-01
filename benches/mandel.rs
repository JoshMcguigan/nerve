use criterion::{criterion_group, criterion_main, Criterion};
use test_utils::CompiledBinary;

pub fn criterion_benchmark(c: &mut Criterion) {
    let source_code = include_str!("../bf-examples/mandel.b");
    let compiled_binary = CompiledBinary::new(source_code);

    let input = "";
    c.bench_function("mandel.b", |b| b.iter(|| compiled_binary.run(input)));
}

fn config() -> Criterion {
    Criterion::default().sample_size(10)
}

criterion_group! {
    name = benches;
    config = config();
    targets = criterion_benchmark
}
criterion_main!(benches);
