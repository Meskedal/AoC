use criterion::{criterion_group, criterion_main, Criterion};
use day1::day1_solve;

fn benchmark_solve(c: &mut Criterion) {
    c.bench_function("solve day 1", |b| b.iter(|| day1_solve()));
}

criterion_group!(benches, benchmark_solve);
criterion_main!(benches);