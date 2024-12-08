fn main() {
    let input = day8::input_generator(include_str!("../input/full.txt"));
    let result = day8::part_1(&input);
    println!("Part 1: {}", result);
    let result = day8::part_2(&input);
    println!("Part 2: {}", result); 
}
