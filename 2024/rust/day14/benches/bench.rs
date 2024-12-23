use criterion::{black_box, criterion_group, criterion_main, Criterion};
fn benchmark_solve(c: &mut Criterion) {
	let width = 101;
	let height = 103;
	c.bench_function("Day 14 Input Generator", |b| b.iter(|| day14::input_generator(black_box(include_str!("../input/full.txt")))));
	let input = day14::input_generator(include_str!("../input/full.txt"));  
	c.bench_function("Day 14 Part 1", |b| b.iter(|| day14::part_1(black_box(&input), black_box(width), black_box(height))));
	c.bench_function("Day 14 Part 2", |b| b.iter(|| day14::part_2(black_box(&input))));
}

criterion_group!(benches, benchmark_solve);
criterion_main!(benches);
