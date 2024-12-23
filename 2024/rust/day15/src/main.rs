fn main() {
	let input  = day15::input_generator(include_str!("../input/full.txt"));
	let part_1 = day15::part_1(&input.0, &input.1);
	let part_2 = day15::part_2(&input.0, &input.1);
	println!("Part 1: {}", part_1);
	println!("Part 2: {}", part_2); 
}
