#[test]
fn test_day_7_part_1_example() {
	let input = day7::input_generator(include_str!("../input/example.txt"));
	let expected = 21;
	let result = day7::part_1(&input);
	assert_eq!(result, expected);
}

#[test]
fn test_day_7_part_2_example() {
	let input = day7::input_generator(include_str!("../input/example.txt"));
	let expected = 0;
	let result = day7::part_2(&input);
	assert_eq!(result, expected);
}
