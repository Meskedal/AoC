use regex::Regex;

#[aoc(day3, part1)]
fn part_1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)+").unwrap();
    let captures = re.captures_iter(input);
    let mut multiplication_sum = 0;
    for i in captures {
        let mul_1 = i[1].parse::<i32>().unwrap();
        let mul_2 = i[2].parse::<i32>().unwrap();
        multiplication_sum += mul_1 * mul_2;
    }
    multiplication_sum
}

#[aoc(day3, part2)]
fn part_2(input: &str) -> i32 {
    let mut multiplication_sum = 0;
    multiplication_sum
}