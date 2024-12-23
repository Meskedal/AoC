use std::{i32, ops::{Add, Mul, Rem, Sub, Div}};

use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, Copy, Clone)]
pub struct ClawMachine {
    button_a: Position,
    button_b: Position,
    prize: Position,
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, other: Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<i64> for Position {
    type Output = Position;

    fn add(self, other: i64) -> Position {
        Position {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

impl Div for Position {
    type Output = Position;

    fn div(self, other: Position) -> Position {
        Position {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl Mul<i64> for Position {
    type Output = Position;

    fn mul(self, other: i64) -> Position {
        Position {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Rem for Position {
    type Output = Position;

    fn rem(self, other: Position) -> Position {
        Position {
            x: self.x % other.x,
            y: self.y % other.y,
        }
    }
}

pub fn input_generator(input: &str) -> Vec<ClawMachine> {
    let re = Regex::new(r"(?:.*X(?:\+|=)(\d*), Y(?:\+|=)(\d*))").unwrap();
    input.split("\r\n\r\n").collect::<Vec<&str>>().into_iter().map(|game| {
        let mut positions = re.captures_iter(game);
        let button_a_match = positions.next().unwrap();
        let button_a = Position {
            x: button_a_match[1].parse().unwrap(),
            y: button_a_match[2].parse().unwrap(),
        };
        let button_b_match = positions.next().unwrap();
        let button_b = Position {
            x: button_b_match[1].parse().unwrap(),
            y: button_b_match[2].parse().unwrap(),
        };
        let prize_match = positions.next().unwrap();
        let prize = Position {
            x: prize_match[1].parse().unwrap(),
            y: prize_match[2].parse().unwrap(),
        };
        ClawMachine {
            button_a,
            button_b,
            prize,
        }
    }).collect()
}

pub fn part_1(input: &Vec<ClawMachine>) -> i64 {
    let mut num_tokens = 0;
    let button_a_token_cost = 3;
    let button_b_token_cost = 1;
    let max_iterations = 100;
    for game in input.iter() {
        let mut min_tokens = i64::MAX;

        // println!("Game: {:?}", game);
        for i in 0..max_iterations {
            for j in 0..max_iterations {
                let button_a_weight = i;
                let button_b_weight = j;

                let x = button_a_weight * game.button_a.x + button_b_weight * game.button_b.x;
                if x != game.prize.x {
                    continue;
                }
                let y = button_a_weight * game.button_a.y + button_b_weight * game.button_b.y;
                if  y != game.prize.y {
                    continue;
                }

                let button_a_tokens = button_a_weight * button_a_token_cost;
                let button_b_tokens = button_b_weight * button_b_token_cost; 

                let total_tokens = button_a_tokens + button_b_tokens;
                if total_tokens < min_tokens {
                    min_tokens = total_tokens;
                }
            }
        }

        if min_tokens < i64::MAX {
            num_tokens += min_tokens;
            continue;
        }
    }
    num_tokens
}


pub fn part_2(input: &Vec<ClawMachine>) -> i64 {
    let mut num_tokens = 0;
    let button_a_token_cost = 3;
    let button_b_token_cost = 1;
    let offset: i64 = 10000000000000;
    // let offset: i64 = 0;
    for game in input.iter() {
        let mut iteration = 1;
        let mut a_done = false;
        let mut b_done = false;

        let mut min_tokens = i64::MAX;
        while !a_done || !b_done{
            let remainder_a = game.prize + offset - (game.button_a * iteration);
            let remainder_b = game.prize + offset - (game.button_b * iteration);

            if remainder_a.x < 0 || remainder_a.y < 0 {
                a_done = true;
            }
            if remainder_b.x < 0 || remainder_b.y < 0 {
                b_done = true;
            }
            let zero = Position { x: 0, y: 0 };
            if !a_done && remainder_a % game.button_b == zero {
                let button_a_tokens = iteration * button_a_token_cost;
                let button_b_tokens = (remainder_a / game.button_b).x * button_b_token_cost;
                let tokens = button_a_tokens + button_b_tokens;
                if tokens < min_tokens {
                    min_tokens = tokens;
                }
                a_done = true;
            }

            if !b_done && remainder_b % game.button_a == zero {
                let button_a_tokens = (remainder_b / game.button_a).x * button_a_token_cost;
                let button_b_tokens = iteration * button_b_token_cost;
                let tokens = button_a_tokens + button_b_tokens;
                if tokens < min_tokens {
                    min_tokens = tokens;
                }
                b_done = true;
            }
            iteration += 1;

        }
        if min_tokens < i64::MAX {
            num_tokens += min_tokens;
            continue;
        }
    }
    num_tokens
}


// pub fn part_2(input: &Vec<ClawMachine>) -> i32 {
//     let mut num_tokens = 0;
//     let button_a_token_cost = 3;
//     let button_b_token_cost = 1;
//     let offset: i64 = 10000000000000;
//     for game in input.iter() {
//         let mut iteration = 1;
//         let mut a_done = false;
//         let mut b_done = false;
//         while !a_done || !b_done{
//             let remainder_a = game.prize - (game.button_a * iteration);
//             let remainder_b = game.prize - (game.button_b * iteration);

//             if remainder_a.x < 0 || remainder_a.y < 0 {
//                 a_done = true;
//             }
//             if remainder_b.x < 0 || remainder_b.y < 0 {
//                 b_done = true;
//             }
//             let zero = Position { x: 0, y: 0 };
//             if !a_done && remainder_a % game.button_b == zero {
//                 let button_a_tokens = iteration * button_a_token_cost;
//                 let button_b_tokens = (remainder_a / game.button_b).x * button_b_token_cost;
//                 num_tokens += button_a_tokens + button_b_tokens;
//                 a_done = true;
//                 break;
//             }

//             if !b_done && remainder_b % game.button_a == zero {
//                 let button_a_tokens = (remainder_b / game.button_a).x * button_a_token_cost;
//                 let button_b_tokens = iteration * button_b_token_cost;
//                 num_tokens += button_a_tokens + button_b_tokens;
//                 b_done = true;
//                 break;
//             }
//             iteration += 1;

//         }
//         let mut min_tokens = i32::MAX;
//         if min_tokens < i32::MAX {
//             num_tokens += min_tokens;
//             continue;
//         }
//     }
//     num_tokens
// }