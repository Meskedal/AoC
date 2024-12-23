#[test]
fn test_day_18_part_1_example() {
	let input = day18::input_generator(include_str!("../input/example.txt"));
	let expected = 480;
	let result = day18::part_1(&input);
	assert_eq!(result, expected);
}

#[test]
fn test_day_18_part_2_example() {
	let input = day18::input_generator(include_str!("../input/example.txt"));
	let expected = 0;
	let result = day18::part_2(&input);
	assert_eq!(result, expected);
}
