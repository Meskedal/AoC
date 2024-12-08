fn main() {
    let input = day4::input_generator(include_str!("../input/full.txt"));
    let result = day4::part_1(&input);
    println!("Part 1: {}", result);
    let result = day4::part_2(&input);
    println!("Part 2: {}", result); 
}
