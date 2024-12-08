#[test]
fn test_day_04_part_1_example() {
    let input = day4::input_generator(include_str!("../input/example.txt"));
    let expected = 18;
    let result = day4::part_1(&input);
    assert_eq!(result, expected);
}

#[test]
fn test_day_04_part_2_example() {
    let input = day4::input_generator(include_str!("../input/example.txt"));
    let expected = 9;
    let result = day4::part_2(&input);
    assert_eq!(result, expected);
}