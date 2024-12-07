use core::num;
use std::collections::HashMap;

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Vec<(i32, Vec<i32>)> {
    // Create a character matrix based on each row and column of the input string 
    input.lines().map(|l| {
        let parts = l.split(": ").collect::<Vec<&str>>();
        let test_value = parts[0].parse().unwrap();
        let numbers = parts[1].split(" ").map(|n| n.parse().unwrap()).collect();
        (test_value, numbers)
    }).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operators {
    ADD, 
    MUL,
    INVALID
}

impl Operators {
    fn variants () -> Vec<Operators> {
        vec![Operators::ADD, Operators::MUL]
    }
}

impl From<i32> for Operators {
    fn from(value: i32) -> Self {
        match value {
            0 => Operators::ADD,
            1 => Operators::MUL,
            _ => Operators::INVALID
        }
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn mul(a: i32, b: i32) -> i32 {
    a * b
}

#[aoc(day7, part1)]
fn part_1(input: &Vec<(i32, Vec<i32>)>) -> i32 {
    let mut count = 0;
    for (test_value, numbers) in input.iter() {

    }
    count
}
