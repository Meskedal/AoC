#[test]
fn test_day_6_part_1_example() {
	let input = day6::input_generator(include_str!("../input/example.txt"));
	let expected = 4277556;
	let result = day6::part_1(&input);
	assert_eq!(result, expected);
}

#[test]
fn test_day_6_part_2_example() {
	let input = day6::input_generator(include_str!("../input/example.txt"));
	let expected = 0;
	let result = day6::part_2(&input);
	assert_eq!(result, expected);
}
