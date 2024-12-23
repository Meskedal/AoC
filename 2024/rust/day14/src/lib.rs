use std::{collections::HashMap, ops::{Add, Sub, Mul}};

use regex::Regex;


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vector {
	x: i32,
	y: i32,
}

impl Add for Vector {
	type Output = Vector;
	fn add(self, other: Vector) -> Vector {
		Vector {x: self.x + other.x, y: self.y + other.y}
	}
}

impl Mul<i32> for Vector {
	type Output = Vector;
	fn mul(self, other: i32) -> Vector {
		Vector {x: self.x * other, y: self.y * other}
	}
}

impl Add for &Vector {
	type Output = Vector;
	fn add(self, other: &Vector) -> Vector {
		Vector {x: self.x + other.x, y: self.y + other.y}
	}
}

impl Sub for Vector {
	type Output = Vector;
	fn sub(self, other: Vector) -> Vector {
		Vector {x: self.x - other.x, y: self.y - other.y}
	}
}

#[derive(Debug, Copy, Clone)]
pub struct Guard {
	pos: Vector,
	vel: Vector,
}

pub fn input_generator(input: &str) -> Vec<Guard> {
	let re = Regex::new(r"p=(\d*),(\d*) v=(-?\d*),(-?\d*)").unwrap();
	input.lines().into_iter().map(|line| { 
		let cap = re.captures_iter(line).next().unwrap();
		Guard {
			pos: Vector {x: cap[1].parse().unwrap(), y: cap[2].parse().unwrap()},
			vel: Vector {x: cap[3].parse().unwrap(), y: cap[4].parse().unwrap()},
		}
	}).collect()
}

fn update_position(pos: &Vector, vel: &Vector, width: i32, height: i32) -> Vector {
	let mut pos_updated = pos + vel;
	if pos_updated.x < 0 {
		pos_updated.x = width - pos_updated.x.abs() % width;
	}

	if pos_updated.y < 0 {
		pos_updated.y = height - pos_updated.y.abs() % height;
	}

	if pos_updated.x >= width {
		pos_updated.x = pos_updated.x.abs() % width;
	}

	if pos_updated.y >= height {
		pos_updated.y = pos_updated.y.abs() % height;
	}

	pos_updated
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
enum Quadrant {
	UL,
	UR,
	DL,
	DR
}

fn get_quadrant(pos: &Vector, width: i32, height: i32) -> Option<Quadrant> {
	if pos.x == width/2 || pos.y == height/2 {
		return None
	}

	if pos.y < height/2 {
		return if pos.x < width/2 {Some(Quadrant::DL) } else {Some(Quadrant::DR)};
	} else {
		return if pos.x < width/2 {Some(Quadrant::UL) } else {Some(Quadrant::UR)};
	}
}

pub fn part_1(input: &Vec<Guard>, width: i32, height: i32) -> i32 {
	let iterations_max = 100;

	let mut guard_map: HashMap<Vector, i32> = HashMap::new();
	let mut quadrant_count: HashMap<Quadrant, i32> = HashMap::new();
	for guard in input {
		let position_updated = update_position(&guard.pos, &(guard.vel*iterations_max), width, height);

		if let Some(quadrant) = get_quadrant(&position_updated, width, height) {
			if let Some(count) = quadrant_count.get(&quadrant) {
				quadrant_count.insert(quadrant, count + 1);

			} else {
				quadrant_count.insert(quadrant, 1);
			}
		}
	}

	quadrant_count.iter().fold(1, |x, (_, &y)| {x*y})
}

fn print_matrix(matrix: &Vec<Vec<i32>>) {
	println!();
	for x in 0..matrix.len() {
		for y in 0..matrix[0].len() {
			print!("{}",matrix[x][y]);
		}
		println!();
	}
}


fn within_bounds(x: i32, y: i32, width: i32, height: i32) -> bool {
    if x < 0 || x >= width || y < 0 || y >= height {
		return false;
	}
	true
}

fn get_christmas_tree_depth(x: i32, y: i32, matrix: &Vec<Vec<i32>>) -> i32 {
	let height = matrix.len() as i32;
	let width = matrix[0].len() as i32;

	let mut depth = 0;
	'inner: loop {
		let y = y + depth;
		let x_min = x - (2*depth);
		let x_max = x + (2*depth);
		for x in x_min..=x_max {
			if !within_bounds(x, y, width, height) {
				break 'inner;
			}
			if matrix[y as usize][x as usize] == 0 {
				break 'inner;
			}
		}
		depth += 1;
	}
	depth
}

pub fn part_2(input: &Vec<Guard>) -> i32 {
	let width = 101;
	let height = 103;

	let mut iteration = 0;
	'main: loop {
		let mut guard_matrix = vec![vec![0; width]; height];
		iteration += 1;
		for guard in input {
			let position_updated = update_position(&guard.pos, &(guard.vel*iteration), width as i32, height as i32);
			guard_matrix[position_updated.y as usize][position_updated.x as usize] += 1;
		}

		for y in 0..height {
			for x in 0..width {
				let depth = get_christmas_tree_depth(x as i32, y as i32, &guard_matrix);
				if depth > 5 {
					// print_matrix(&guard_matrix);
					break 'main;	
				}
			}
		}
	}
	iteration

}
