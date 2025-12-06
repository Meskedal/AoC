#[test]
fn test_day_1_part_1_example() {
	let input = day1::input_generator(include_str!("../input/example.txt"));
	let expected = 3;
	let result = day1::part_1(&input);
	assert_eq!(result, expected);
}

#[test]
fn test_day_1_part_2_example() {
	let input = day1::input_generator(include_str!("../input/example.txt"));
	let expected = 6;
	let result = day1::part_2(&input);
	assert_eq!(result, expected);
}

#[test]
fn test_day_1_part_2_example_2() {
	let input = day1::input_generator(include_str!("../input/example2.txt"));
	let expected = 6;
	let result = day1::part_2(&input);
	assert_eq!(result, expected);
}