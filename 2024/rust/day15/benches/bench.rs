use criterion::{black_box, criterion_group, criterion_main, Criterion};
fn benchmark_solve(c: &mut Criterion) {
	c.bench_function("Day 15 Input Generator", |b| b.iter(|| day15::input_generator(black_box(include_str!("../input/full.txt")))));
	let input = day15::input_generator(include_str!("../input/full.txt"));  
	c.bench_function("Day 15 Part 1", |b| b.iter(|| day15::part_1(black_box(&input.0), black_box(&input.1))));
	c.bench_function("Day 15 Part 2", |b| b.iter(|| day15::part_2(black_box(&input.0), black_box(&input.1))));
}

criterion_group!(benches, benchmark_solve);
criterion_main!(benches);
