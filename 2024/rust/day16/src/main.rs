fn main() {
	let input  = day16::input_generator(include_str!("../input/full.txt"));
	let part_1 = day16::part_1(&input);
	let part_2 = day16::part_2(&input);
	println!("Part 1: {}", part_1);
	println!("Part 2: {}", part_2); 
}
