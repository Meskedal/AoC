use std::collections::HashSet;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

#[aoc_generator(day6)]
fn input_generator(input: &str) -> Vec<Vec<char>> {
    // Create a character matrix based on each row and column of the input string 
    let char_matrix = input.lines().map(|l| l.chars().collect()).collect();
    char_matrix
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vector {
    row: i32,
    col: i32,
}

impl Vector {
    fn rotate_clockwise(&self) -> Vector {
        Vector {
            row: self.col,
            col: -self.row,
        }
    }
}

fn print_matrix(matrix: &Vec<Vec<char>>) {
    print!("\x1B[2J\x1B[1;1H");
    for row in matrix {
        for &c in row {
            print!("{}", c);
        }
        println!();
    }
    io::stdout().flush().unwrap();
}

#[aoc(day6, part1)]
fn part_1(char_matrix: &Vec<Vec<char>>) -> i32 {
    let rows_min: i32 = 0;
    let cols_min: i32 = 0;
    let rows_max = char_matrix.len() as i32;
    let cols_max = char_matrix[0].len() as i32;

    // 1. Identify the position of the guard 
    let guard_char = '^';
    let mut guard_position = Vector { row: 0, col: 0 };

    if let Some((i, row)) = char_matrix.iter().enumerate().find(|(_, row)| row.contains(&guard_char)) {
        if let Some(j) = row.iter().position(|&c| c == guard_char) {
            guard_position = Vector { row: i as i32, col: j as i32 };
        }
    }

    // 2. Assign initial guard direction and store initial position
    let mut guard_unit_direction = Vector { row: -1, col: 0 };
    let mut distinct_steps =  HashSet::from([guard_position]);

    // 3. Continue in the direction until we reach an obstacle # and we can not move through it.
    let obstacle_char = '#';

    let mut char_matrix_debug = char_matrix.clone();
    loop {
        // Update the debug matrix
        char_matrix_debug[guard_position.row as usize][guard_position.col as usize] = 'X';

        let row = guard_position.row + guard_unit_direction.row;
        let col = guard_position.col + guard_unit_direction.col;

        if !(row >= rows_min && row < rows_max && col >= cols_min && col < cols_max) {
            // We are outside boundaries and done 
            break;
        }

        if char_matrix[row as usize][col as usize] == obstacle_char {
            // Rotate the guard unit direction clockwise
            guard_unit_direction = guard_unit_direction.rotate_clockwise();
            continue;
        } 
        // Move the guard in the current direction
        guard_position = Vector { row, col };
        distinct_steps.insert(guard_position);
        // Update the debug matrix
        char_matrix_debug[guard_position.row as usize][guard_position.col as usize] = '^';
    }

    print_matrix(&char_matrix_debug);

    distinct_steps.len() as i32 
}

#[aoc(day6, part2)]
fn part_2(char_matrix: &Vec<Vec<char>>) -> i32 {
    let rows_min: i32 = 0;
    let cols_min: i32 = 0;
    let rows_max = char_matrix.len() as i32;
    let cols_max = char_matrix[0].len() as i32;

    // To create a loop, we have to iterate through the whole matrix and find 3 obstacles that can be used to complete a 4 sided loop by placing an obstacle 

    for i in rows_min..rows_max {
        for j in cols_min..cols_max {

        }
    }
    0
}