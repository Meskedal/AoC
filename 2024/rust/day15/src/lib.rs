use std::ops::{Add, Mul};
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vector {
	row: i32,
	col: i32,
}

impl Add for Vector {
	type Output = Vector;
	fn add(self, other: Vector) -> Vector {
		Vector {row: self.row + other.row, col: self.col + other.col}
	}
}

impl Add for &Vector {
	type Output = Vector;
	fn add(self, other: &Vector) -> Vector {
		Vector {row: self.row + other.row, col: self.col + other.col}
	}
}

impl Mul<i32> for Vector {
	type Output = Vector;
	fn mul(self, other: i32) -> Vector {
		Vector {row: self.row * other, col: self.col * other}
	}
}

const ROBOT_CHAR: char = '@';
const BOX_CHAR: char = 'O';
const WALL_CHAR: char = '#';
const EMPTY_CHAR: char = '.';

fn print_matrix(matrix: &Vec<Vec<char>>) {
	println!();
	for row in 0..matrix.len() {
		for col in 0..matrix[0].len() {
			print!("{}",matrix[row][col]);
		}
		println!();
	}
}

pub fn input_generator(input: &str) -> (Vec<Vec<char>>, Vec<Vector>) {
	let split: Vec<&str> = input.split("\r\n\r\n").collect();
	let map = split[0].lines().map(|line| {line.chars().collect()}).collect();
	let moves = split[1].replace("\r\n", "").chars().map(|c| {
		match c {
			'^' => Vector {row: -1, col: 0},
			'v' => Vector {row: 1, col: 0},
			'>' => Vector {row: 0, col: 1},
			'<' => Vector {row: 0, col: -1},
			_ => {
				// Should never happen so we just panic in this case
				panic!()
			}
		}
	}).collect();
	(map, moves)
}

fn find_robot_position(map: &Vec<Vec<char>>, robot_char: char) -> Vector {
	for row in 0..map.len() {
		for col in 0..map[0].len() {
			if map[row][col] == robot_char {
				return Vector {row: row as i32, col: col as i32};
			}
		}
	}
	// Should never happen so just panic in this case
	panic!();
}

pub fn part_1(map: &Vec<Vec<char>>, moves: &Vec<Vector>) -> i32 {
	let mut map = map.clone();
	let mut robot_position = find_robot_position(&map, ROBOT_CHAR);
	for mv in moves.iter() {
		let new_position = robot_position + *mv;
		let new_position_char = map[new_position.row as usize][new_position.col as usize];

		match new_position_char {
			WALL_CHAR => continue,
			EMPTY_CHAR => {
				map[robot_position.row as usize][robot_position.col as usize] = EMPTY_CHAR;
				map[new_position.row as usize][new_position.col as usize] = ROBOT_CHAR;
				robot_position = new_position;
			},
			BOX_CHAR => {
				// We search for the open slot in the current direction 
				let mut boxes = 1;
				loop {
					let swap_position = robot_position + (*mv * (boxes + 1)) ;
					let swap_position_char = map[swap_position.row as usize][swap_position.col as usize];
					if swap_position_char == WALL_CHAR {
						// Nothing to swap against, we've hit a wall 
						break;
					}
					if swap_position_char == EMPTY_CHAR {
						// Found the swap position, this is where we swap 
						map[swap_position.row as usize][swap_position.col as usize] = BOX_CHAR;
						map[robot_position.row as usize][robot_position.col as usize] = EMPTY_CHAR;
						map[new_position.row as usize][new_position.col as usize] = ROBOT_CHAR;
						robot_position = new_position;
						break;
					}
					// Finally, this is the case where we found another box, then we just iterate again
					boxes += 1;
				}
			},
			_ => panic!("Unexpected Character: {:?}", new_position_char)
		}
	}
	let row_max = map.len() - 1;
	let col_max = map[0].len() - 1;
	let mut sum = 0;
	for row in 0..=row_max {
		for col in 0..=col_max {
			if map[row][col] != BOX_CHAR {
				continue;
			}
			sum += 100*row + col;
		}
	}
	sum as i32
}

pub fn part_2(map: &Vec<Vec<char>>, moves: &Vec<Vector>) -> i32 {
	let robot_char = '@';
	let box_char = 'O';
	let wall_char = '#';
	print_matrix(map);
	let robot_position = find_robot_position(map, robot_char);
	0
}
