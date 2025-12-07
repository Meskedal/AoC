fn main() {
	let input  = day6::input_generator(include_str!("../input/full.txt"));
	let part_1 = day6::part_1(&input);
	let input2  = day6::input_generator2(include_str!("../input/full.txt"));
	let part_2 = day6::part_2(&input2);
	println!("Part 1: {}", part_1);
	println!("Part 2: {}", part_2); 
}
