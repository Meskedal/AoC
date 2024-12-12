#[test]
fn test_day_12_part_1_example() {
    let input = day12::input_generator(include_str!("../input/example.txt"));
    let expected = 1930;
    let result = day12::part_1(&input);
    assert_eq!(result, expected);
}

#[test]
fn test_day_12_part_2_example() {
    let input = day12::input_generator(include_str!("../input/example.txt"));
    let expected = 4;
    let result = day12::part_2(&input);
    assert_eq!(result, expected);
}