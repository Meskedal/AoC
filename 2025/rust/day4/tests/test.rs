#[test]
fn test_day_4_part_1_example() {
	let input = day4::input_generator(include_str!("../input/example.txt"));
	let expected = 13;
	let result = day4::part_1(&input);
	assert_eq!(result, expected);
}

#[test]
fn test_day_4_part_2_example() {
	let input = day4::input_generator(include_str!("../input/example.txt"));
	let expected = 0;
	let result = day4::part_2(&input);
	assert_eq!(result, expected);
}
