#[test]
fn test_day_17_part_1_example() {
	let input = day17::input_generator(include_str!("../input/example.txt"));
	let expected = 480;
	let result = day17::part_1(&input);
	assert_eq!(result, expected);
}

#[test]
fn test_day_17_part_2_example() {
	let input = day17::input_generator(include_str!("../input/example.txt"));
	let expected = 0;
	let result = day17::part_2(&input);
	assert_eq!(result, expected);
}
