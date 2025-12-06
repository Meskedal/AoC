#[test]
fn test_day_5_part_1_example() {
	let input = day5::input_generator(include_str!("../input/example.txt"));
	let expected = 3;
	let result = day5::part_1(&input);
	assert_eq!(result, expected);
}

#[test]
fn test_day_5_part_2_example() {
	let input = day5::input_generator(include_str!("../input/example.txt"));
	let expected = 0;
	let result = day5::part_2(&input);
	assert_eq!(result, expected);
}
