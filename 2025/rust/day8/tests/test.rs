#[test]
fn test_day_8_part_1_example() {
	let input = day8::input_generator(include_str!("../input/example.txt"));
	let expected = 40;
	let result = day8::part_1(&input, 10);
	assert_eq!(result, expected);
}

#[test]
fn test_day_8_part_2_example() {
	let input = day8::input_generator(include_str!("../input/example.txt"));
	let expected = 25272;
	let result = day8::part_2(&input);
	assert_eq!(result, expected);
}
