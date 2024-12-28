#[test]
fn test_day_16_part_1_example() {
	let input = day16::input_generator(include_str!("../input/example.txt"));
	let expected = 7036;
	let result = day16::part_1(&input);
	assert_eq!(result, expected);
}

#[test]
fn test_day_16_part_1_example_2() {
	let input = day16::input_generator(include_str!("../input/example2.txt"));
	let expected = 11048;
	let result = day16::part_1(&input);
	assert_eq!(result, expected);
}

#[test]
fn test_day_16_part_2_example() {
	let input = day16::input_generator(include_str!("../input/example.txt"));
	let expected = 45;
	let result = day16::part_2(&input);
	assert_eq!(result, expected);
}

#[test]
fn test_day_16_part_2_example_2() {
	let input = day16::input_generator(include_str!("../input/example2.txt"));
	let expected = 64;
	let result = day16::part_2(&input);
	assert_eq!(result, expected);
}
