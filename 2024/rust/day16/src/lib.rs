use std::collections::HashMap;

use vector2d::Vector2D;


struct Reindeer {
	pos: Vector2D<i32>,
	dir: Vector2D<i32>,
}


fn rotate_clockwise<T: Copy + std::ops::Neg<Output = T>>(dir: Vector2D<T>) -> Vector2D<T> {
	Vector2D::new(dir.y, -dir.x)
}

fn rotate_counter_clockwise<T: Copy + std::ops::Neg<Output = T>>(dir: Vector2D<T>) -> Vector2D<T> {
	Vector2D::new(-dir.y, dir.x)
}

const START_CHAR: char = 'S';
const END_CHAR: char = 'E';
const WALL_CHAR: char = '#';
const EMPTY_CHAR: char = '.';

const UP: Vector2D<i32> = Vector2D {x: 0, y: -1};
const DOWN: Vector2D<i32> = Vector2D {x: 0, y: 1};
const LEFT: Vector2D<i32> = Vector2D {x: -1, y: 0};
const RIGHT: Vector2D<i32> = Vector2D {x: 1, y: 0};

pub fn input_generator(input: &str) -> Vec<Vec<char>> {
	input.lines().map(|l| {l.chars().collect()}).collect()
}

pub fn find_char(maze: &Vec<Vec<char>>, c: char) -> Vector2D<usize> {
	let (x, y, _) = maze.iter().enumerate().flat_map(|(y, inner_vector)| {
        inner_vector.iter().enumerate().map(move |(x, &value)| (x, y, value))
    }).find(|&(_, _, value)| value == c).unwrap();
	Vector2D::new(x, y)
}

#[derive(Copy, Clone)]
struct PathNode {
	left: bool,
	right: bool,
	up: bool,
	down: bool,
}

pub fn part_1(maze: &Vec<Vec<char>>) -> i32 {
	let start= find_char(maze, START_CHAR);
	let end = find_char(maze, END_CHAR);

	let mut maze_nodes = vec![vec![PathNode{left: false, right: false, up: false, down: false}; maze[0].len()]; maze.len()];
	for y in 1..maze.len() - 1 {
		for x in 1..maze[0].len()-1 {
			let up_position = Vector2D {x: x, y: y} + UP.as_usizes();
			let left_position = Vector2D {x: x, y: y} + LEFT.as_usizes();
			let right_position = Vector2D {x: x, y: y} + RIGHT.as_usizes();
			let down_position = Vector2D {x: x, y: y} + DOWN.as_usizes();
			if maze[up_position.y][up_position.x] == EMPTY_CHAR {
				maze_nodes[y][x].up = true;
			}
			if maze[left_position.y][left_position.x] == EMPTY_CHAR {
				maze_nodes[y][x].left = true;
			}
			if maze[right_position.y][right_position.x] == EMPTY_CHAR {
				maze_nodes[y][x].right = true;
			}
			if maze[down_position.y][down_position.x] == EMPTY_CHAR {
				maze_nodes[y][x].down = true;
			}
		}
	}

	println!("{:?}, {:?}", start, end);
	0
}

pub fn part_2(maze: &Vec<Vec<char>>) -> i32 {
	0
}
