#[test]
fn test_day_10_part_1_example() {
    let input = day10::input_generator(include_str!("../input/example.txt"));
    let expected = 36;
    let result = day10::part_1(&input);
    assert_eq!(result, expected);
}

#[test]
fn test_day_10_part_2_example() {
    let input = day10::input_generator(include_str!("../input/example.txt"));
    let expected = 4;
    let result = day10::part_2(&input);
    assert_eq!(result, expected);
}