fn main() {
	let width = 101;
	let height = 103;
	let input  = day14::input_generator(include_str!("../input/full.txt"));
	let part_1 = day14::part_1(&input, width, height);
	let part_2 = day14::part_2(&input);
	println!("Part 1: {}", part_1);
	println!("Part 2: {}", part_2); 
}
