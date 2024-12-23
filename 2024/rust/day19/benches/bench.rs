use criterion::{black_box, criterion_group, criterion_main, Criterion};
fn benchmark_solve(c: &mut Criterion) {
	c.bench_function("Day 19 Input Generator", |b| b.iter(|| day19::input_generator(black_box(include_str!("../input/full.txt")))));
	let input = day19::input_generator(include_str!("../input/full.txt"));  
	c.bench_function("Day 19 Part 1", |b| b.iter(|| day19::part_1(black_box(&input))));
	c.bench_function("Day 19 Part 2", |b| b.iter(|| day19::part_2(black_box(&input))));
}

criterion_group!(benches, benchmark_solve);
criterion_main!(benches);
