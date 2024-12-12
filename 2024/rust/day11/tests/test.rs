#[test]
fn test_day_11_part_1_example() {
    let input = day11::input_generator(include_str!("../input/example.txt"));
    let expected = 55312;
    let result = day11::part_1(&input);
    assert_eq!(result, expected);
}

#[test]
fn test_day_11_part_2_example() {
    let input = day11::input_generator(include_str!("../input/example.txt"));
    let expected = 4;
    let result = day11::part_2(&input);
    assert_eq!(result, expected);
}