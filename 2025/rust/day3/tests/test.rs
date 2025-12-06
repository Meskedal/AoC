#[test]
fn test_day_3_part_1_example() {
	let input = day3::input_generator(include_str!("../input/example.txt"));
	let expected = 357;
	let result = day3::part_1(&input);
	assert_eq!(result, expected);
}

#[test]
fn test_day_3_part_2_example() {
	let input = day3::input_generator(include_str!("../input/example.txt"));
	let expected = 3121910778619;
	let result = day3::part_2(&input);
	assert_eq!(result, expected);
}
