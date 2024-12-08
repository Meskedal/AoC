#[test]
fn test_day_05_part_1_example() {
    let input = day5::input_generator(include_str!("../input/example.txt"));
    let expected = 143;
    let result = day5::part_1(&input);
    assert_eq!(result, expected);
}

#[test]
fn test_day_05_part_2_example() {
    let input = day5::input_generator(include_str!("../input/example.txt"));
    let expected = 123;
    let result = day5::part_2(&input);
    assert_eq!(result, expected);
}