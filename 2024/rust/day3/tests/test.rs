#[test]
fn test_day_03_part_1_example() {
    let input = include_str!("../input/example.txt");
    let expected =  161;
    let result = day3::part_1(&input);
    assert_eq!(result, expected);
}

#[test]
fn test_day_03_part_2_example() {
    let input = include_str!("../input/example.txt");
    let expected = 48;
    let result = day3::part_2(&input);
    assert_eq!(result, expected);
}