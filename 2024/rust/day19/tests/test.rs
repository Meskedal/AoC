#[test]
fn test_day_19_part_1_example() {
	let input = day19::input_generator(include_str!("../input/example.txt"));
	let expected = 480;
	let result = day19::part_1(&input);
	assert_eq!(result, expected);
}

#[test]
fn test_day_19_part_2_example() {
	let input = day19::input_generator(include_str!("../input/example.txt"));
	let expected = 0;
	let result = day19::part_2(&input);
	assert_eq!(result, expected);
}
