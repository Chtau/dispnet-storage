use criterion::{criterion_group, criterion_main, Criterion};

fn b1() {
    let a = 1;
    assert_eq!(a, 1);
}


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("TODO", |b| b.iter(|| b1()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);