#[test]
fn test_day_9_part_1_example() {
    let input = day9::input_generator(include_str!("../input/example.txt"));
    let expected = 1928;
    let result = day9::part_1(&input);
    assert_eq!(result, expected);
}

#[test]
fn test_day_9_part_1_v2_example() {
    let input = day9::input_generator(include_str!("../input/example.txt"));
    let expected = 1928;
    let result = day9::part_1_v2(&input);
    assert_eq!(result, expected);
}

#[test]
fn test_day_9_part_2_example() {
    let input = day9::input_generator(include_str!("../input/example.txt"));
    let expected = 2858;
    let result = day9::part_2(&input);
    assert_eq!(result, expected);
}