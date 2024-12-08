#[test]
fn test_day_06_part_1_example() {
    let input = day6::input_generator(include_str!("../input/example.txt"));
    let expected = 41;
    let result = day6::part_1(&input);
    assert_eq!(result, expected);
}

#[test]
fn test_day_06_part_2_example() {
    let input = day6::input_generator(include_str!("../input/example.txt"));
    let expected = 6;
    let result = day6::part_2(&input);
    assert_eq!(result, expected);
}