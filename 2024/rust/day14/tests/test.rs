#[test]
fn test_day_14_part_1_example() {
	let width = 11;
	let height = 7;
	let input = day14::input_generator(include_str!("../input/example.txt"));
	let expected = 12;
	let result = day14::part_1(&input, width, height);
	assert_eq!(result, expected);
}