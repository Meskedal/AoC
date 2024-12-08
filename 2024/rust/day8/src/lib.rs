use std::collections::{HashMap, HashSet};

pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn print_matrix(matrix: &Vec<Vec<char>>) {
    for row in matrix {
        for &c in row {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: i32,
    col: i32,
}

fn within_bounds(Position { row, col }: Position, row_min: i32, col_min: i32, row_max: i32, col_max: i32) -> bool {
    row >= row_min && row < row_max && col >= col_min && col < col_max
}

pub fn part_1(input: &Vec<Vec<char>>) -> i32 {
    let row_min = 0;
    let col_min = 0;
    let row_max = input.len() as i32;
    let col_max = input[0].len() as i32;

    let mut antenna_map: HashMap<char, Vec<Position>> = HashMap::new();
    // 1. Map out all the locations of the different antennas 
    for (i, row) in input.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if input[i][j] != '.' {
                if let Some(positions) = antenna_map.get_mut(&c) {
                    positions.push(Position { row: i as i32, col: j as i32 });
                    continue;
                }
                antenna_map.insert(c, vec![Position { row: i as i32, col: j as i32 }]);
            }
        }
    }
    let mut antinodes = HashSet::new();
    // let mut input_debug = input.clone();
    for (_, positions) in antenna_map.iter() {
        for &position in positions.iter() {
            for &other_position in positions.iter() {
                if position == other_position {
                    continue;
                }

                let antinode_offset = Position { row: (other_position.row - position.row)*2, col: (other_position.col - position.col)*2};
                let antinode = Position { row: position.row + antinode_offset.row, col: position.col + antinode_offset.col};
                if within_bounds(antinode, row_min, col_min, row_max, col_max) {
                    antinodes.insert(antinode);
                    // input_debug[antinode.row as usize][antinode.col as usize] = '#';
                }
            }
        }
    }

    
    // print_matrix(&input_debug);
    antinodes.len() as i32
}

pub fn part_2(input: &Vec<Vec<char>>) -> i32 {
    0
}