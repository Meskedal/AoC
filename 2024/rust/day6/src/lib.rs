use std::collections::HashSet;
use std::hash::Hash;

pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    // Create a character matrix based on each row and column of the input string 
    let char_matrix = input.lines().map(|l| l.chars().collect()).collect();
    char_matrix
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vector {
    row: i32,
    col: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pose {
    position: Vector,
    direction: Vector,
}

impl Vector {
    fn rotate_clockwise(&self) -> Vector {
        Vector {
            row: self.col,
            col: -self.row,
        }
    }
}

pub fn part_1(char_matrix: &Vec<Vec<char>>) -> i32 {
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
    let mut guard_direction = Vector { row: -1, col: 0 };
    let mut distinct_steps =  HashSet::from([guard_position]);

    // 3. Continue in the direction until we reach an obstacle # and we can not move through it.
    let obstacle_char = '#';

    loop {
        let row = guard_position.row + guard_direction.row;
        let col = guard_position.col + guard_direction.col;

        if !(row >= rows_min && row < rows_max && col >= cols_min && col < cols_max) {
            // We are outside boundaries and done 
            break;
        }

        if char_matrix[row as usize][col as usize] == obstacle_char {
            // Rotate the guard unit direction clockwise
            guard_direction = guard_direction.rotate_clockwise();
            continue;
        } 
        // Move the guard in the current direction
        guard_position = Vector { row, col };
        distinct_steps.insert(guard_position);
    }
    distinct_steps.len() as i32 
}

fn does_obstruction_cause_loop(char_matrix: &Vec<Vec<char>>, guard_position: Vector, guard_unit_direction: Vector, obstruction_position: Vector) -> bool {
    let rows_min: i32 = 0;
    let cols_min: i32 = 0;
    let rows_max = char_matrix.len() as i32;
    let cols_max = char_matrix[0].len() as i32;

    let mut guard_pose = Pose { position: guard_position, direction: guard_unit_direction };

    let mut distinct_poses =  HashSet::new();

    let obstacle_char = '#';
    loop {
        if distinct_poses.contains(&guard_pose) {
            return true;
        }
        distinct_poses.insert(guard_pose); 

        let row = guard_pose.position.row + guard_pose.direction.row;
        let col = guard_pose.position.col + guard_pose.direction.col;

        // If we arrive at a previously used pose (position and direction) we have a loop
        if !(row >= rows_min && row < rows_max && col >= cols_min && col < cols_max) {
            // We are outside boundaries and done 
            break;
        }

        // Rotate if the next position is an obstacle or the obstruction position
        if char_matrix[row as usize][col as usize] == obstacle_char  || (row == obstruction_position.row && col == obstruction_position.col) {
            // Rotate the guard unit direction clockwise
            guard_pose.direction = guard_pose.direction.rotate_clockwise();
            continue;
        } 
        // Move the guard in the current direction
        guard_pose.position = Vector { row, col };
    }
    false
}

pub fn part_2(char_matrix: &Vec<Vec<char>>) -> i32 {
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
    let mut guard_direction = Vector { row: -1, col: 0 };
    let mut distinct_obstructions =  HashSet::new();

    let guard_position_initial = guard_position;    
    let guard_direction_initial = guard_direction;

    // 3. Continue in the direction until we reach an obstacle # and we can not move through it.
    let obstacle_char = '#';
    loop {
        // Check if we create a loop along the way if we place an obstruction in the next position
        let row = guard_position.row + guard_direction.row;
        let col = guard_position.col + guard_direction.col;

        if !(row >= rows_min && row < rows_max && col >= cols_min && col < cols_max) {
            // We are outside boundaries and done 
            break;
        }

        let obstruction_position = Vector { row, col };
        if does_obstruction_cause_loop(char_matrix, guard_position_initial, guard_direction_initial, obstruction_position) {
            distinct_obstructions.insert(obstruction_position);
        }

        if char_matrix[row as usize][col as usize] == obstacle_char {
            // Rotate the guard unit direction clockwise
            guard_direction = guard_direction.rotate_clockwise();
            continue;
        } 
        // Move the guard in the current direction
        guard_position = Vector { row, col };
    }
    distinct_obstructions.len() as i32
}