#[test]
fn test_day_2_part_1_example() {
	let input = day2::input_generator(include_str!("../input/example.txt"));
	let expected = 1227775554;
	let result = day2::part_1(&input);
	assert_eq!(result, expected);
}

#[test]
fn test_day_2_part_2_example() {
	let input = day2::input_generator(include_str!("../input/example.txt"));
	let expected = 4174379265;
	let result = day2::part_2(&input);
	assert_eq!(result, expected);
}
