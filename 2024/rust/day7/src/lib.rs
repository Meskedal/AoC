use itertools::Itertools;
pub fn input_generator(input: &str) -> Vec<(i64, Vec<i64>)> {
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
    CONCAT,
    INVALID
}

impl From<i64> for Operators {
    fn from(value: i64) -> Self {
        match value {
            0 => Operators::ADD,
            1 => Operators::MUL,
            2 => Operators::CONCAT,
            _ => Operators::INVALID
        }
    }
}

fn add(a: i64, b: i64) -> i64 {
    a.saturating_add(b)
}

fn mul(a: i64, b: i64) -> i64 {
    a.saturating_mul(b)
}

fn concat(a: i64, b: i64) -> i64 {
    format!("{}{}", a, b).parse().unwrap()
}

pub fn part_1(input: &Vec<(i64, Vec<i64>)>) -> i64 {
    let mut total_sum_test_values = 0;
    for (test_value, numbers) in input.iter() {
        let values = vec![Operators::ADD, Operators::MUL];
        let operators: Vec<Vec<Operators>> = (0..numbers.len() - 1)
        .map(|_| values.iter().cloned())
        .multi_cartesian_product()
        .collect();
        for operator in operators.iter() {
            let result = operator.iter().zip(numbers.iter().skip(1)).fold(numbers[0], |acc, (op, &val)| {
                match op {
                    Operators::ADD => add(acc, val),
                    Operators::MUL => mul(acc, val),
                    Operators::CONCAT => concat(acc, val),
                    _ => {
                        println!("Invalid operator {:?}", op);
                        0
                    }
                }
            });
            // for (i, values) in numbers.windows(2).enumerate() {
            //     result += match operator[i] {
            //         Operators::ADD => add(values[0], values[1]),
            //         Operators::MUL => mul(values[0], values[1]),
            //         _ => 0
            //     };
            // }

            // println!("Result: {}, Expected: {}, Numbers {:?}, Operators: {:?}", result, *test_value, numbers, operator);
            if result == *test_value {
                total_sum_test_values += result;
                break;
            }
        }
    }
    total_sum_test_values
}


pub fn part_2(input: &Vec<(i64, Vec<i64>)>) -> i64 {
    let mut total_sum_test_values = 0;
    for (test_value, numbers) in input.iter() {
        let values = vec![Operators::ADD, Operators::MUL, Operators::CONCAT];
        let operators: Vec<Vec<Operators>> = (0..numbers.len() - 1)
        .map(|_| values.iter().cloned())
        .multi_cartesian_product()
        .collect();
        for operator in operators.iter() {
            let result = operator.iter().zip(numbers.iter().skip(1)).fold(numbers[0], |acc, (op, &val)| {
                match op {
                    Operators::ADD => add(acc, val),
                    Operators::MUL => mul(acc, val),
                    Operators::CONCAT => concat(acc, val),
                    _ => {
                        println!("Invalid operator {:?}", op);
                        0
                    }
                }
            });
            // for (i, values) in numbers.windows(2).enumerate() {
            //     result += match operator[i] {
            //         Operators::ADD => add(values[0], values[1]),
            //         Operators::MUL => mul(values[0], values[1]),
            //         _ => 0
            //     };
            // }

            // println!("Result: {}, Expected: {}, Numbers {:?}, Operators: {:?}", result, *test_value, numbers, operator);
            if result == *test_value {
                total_sum_test_values += result;
                break;
            }
        }
    }
    total_sum_test_values
}
