use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_solve(c: &mut Criterion) {
    let input =include_str!("../input/full.txt");  
    c.bench_function("Day 03 Part 1", |b| b.iter(|| day3::part_1(black_box(&input))));
    c.bench_function("Day 03 Part 2", |b| b.iter(|| day3::part_2(black_box(&input))));
}

criterion_group!(benches, benchmark_solve);
criterion_main!(benches);