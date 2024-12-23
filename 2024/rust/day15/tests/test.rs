#[test]
fn test_day_15_part_1_example() {
	let input = day15::input_generator(include_str!("../input/example.txt"));
	let expected = 2028;
	let result = day15::part_1(&input.0, &input.1);
	assert_eq!(result, expected);
}

#[test]
fn test_day_15_part_1_example_large() {
	let input = day15::input_generator(include_str!("../input/example_large.txt"));
	let expected = 10092;
	let result = day15::part_1(&input.0, &input.1);
	assert_eq!(result, expected);
}

#[test]
fn test_day_15_part_2_example() {
	let input = day15::input_generator(include_str!("../input/example.txt"));
	let expected = 0;
	let result = day15::part_2(&input.0, &input.1);
	assert_eq!(result, expected);
}
