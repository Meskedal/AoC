#[test]
fn test_day_1_part_1_example() {
    let input = include_str!("../input/example.txt");
    let expected = 11;
    let result = day1::part_1(&input);
    assert_eq!(result, expected);
}

#[test]
fn test_day_1_part_2_example() {
    let input = include_str!("../input/example.txt");
    let expected = 31;
    let result = day1::part_2(&input);
    assert_eq!(result, expected);
}