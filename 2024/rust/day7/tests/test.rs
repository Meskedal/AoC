#[test]
fn test_day_07_part_1_example() {
    let input = day7::input_generator(include_str!("../input/example.txt"));
    let expected = 3749;
    let result = day7::part_1(&input);
    assert_eq!(result, expected);
}

#[test]
fn test_day_07_part_2_example() {
    let input = day7::input_generator(include_str!("../input/example.txt"));
    let expected = 11387;
    let result = day7::part_2(&input);
    assert_eq!(result, expected);
}