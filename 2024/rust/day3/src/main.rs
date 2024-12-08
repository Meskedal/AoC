fn main() {
    let input = include_str!("../input/full.txt");
    let result = day3::part_1(&input);
    println!("Part 1: {}", result);
    let result = day3::part_2(&input);
    println!("Part 2: {}", result); 
}
