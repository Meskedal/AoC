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
	min_cost: i32,
}

fn print_cost_matrix(maze_nodes: &Vec<Vec<PathNode>>) {
	for y in 0..maze_nodes.len() {
		for x in 0..maze_nodes[0].len() {
			print!("{}, ", if maze_nodes[y][x].min_cost != i32::MAX {maze_nodes[y][x].min_cost } else {-1});
		}
		println!();
	}
}
fn print_matrix(maze: &Vec<Vec<char>>) {
	for y in 0..maze.len() {
		for x in 0..maze[0].len() {
			print!("{}", maze[y][x]);
		}
		println!();
	}
}

fn get_rotate_cost(current_direction: &Vector2D<i32>, next_direction: &Vector2D<i32>) -> i32 {
	let rotate_cost = 1000;
	if current_direction == next_direction {
		return 0
	}

	if rotate_clockwise(*current_direction) == *next_direction {
		return rotate_cost;
	}

	if rotate_clockwise(rotate_clockwise(*current_direction)) == *next_direction {
		return rotate_cost * 2;
	}

	if rotate_counter_clockwise(*current_direction) == *next_direction {
		return rotate_cost;
	}

	panic!("Unable to rotate direction to valid direction")
}

fn update_nodes(position: &Vector2D<usize>, accumulated_cost: i32, prev_direction: Vector2D<i32>, maze_nodes: &mut Vec<Vec<PathNode>>) {
	let current_node = maze_nodes[position.y][position.x];
	let mut next_node: Vector2D<usize>;

	// If we are exploring a node that has a currently higher cost than the min cost of the node, the there is node need to explore this option. 
	if accumulated_cost > current_node.min_cost {
		return;
	}

	maze_nodes[position.y][position.x].min_cost = accumulated_cost;


	if current_node.up {
		let accumulated_cost = accumulated_cost + get_rotate_cost(&prev_direction, &UP) + 1;
		next_node = (position.as_i32s() + UP).as_usizes();
		update_nodes(&next_node, accumulated_cost, UP, maze_nodes);
	}

	if current_node.down { 
		let accumulated_cost = accumulated_cost + get_rotate_cost(&prev_direction, &DOWN) + 1;
		next_node = (position.as_i32s() + DOWN).as_usizes();
		update_nodes(&next_node, accumulated_cost, DOWN, maze_nodes);
	}

	if current_node.left {
		let accumulated_cost = accumulated_cost + get_rotate_cost(&prev_direction, &LEFT) + 1;
		next_node = (position.as_i32s() + LEFT).as_usizes();
		update_nodes(&next_node, accumulated_cost, LEFT, maze_nodes);
	}
	
	if current_node.right {
		let accumulated_cost = accumulated_cost + get_rotate_cost(&prev_direction, &RIGHT) + 1;
		next_node = (position.as_i32s() + RIGHT).as_usizes();
		update_nodes(&next_node, accumulated_cost, RIGHT, maze_nodes);
	}
}

pub fn part_1(maze: &Vec<Vec<char>>) -> i32 {
	let start= find_char(maze, START_CHAR);
	let end = find_char(maze, END_CHAR);

	let mut maze_nodes = vec![vec![PathNode{left: false, right: false, up: false, down: false, min_cost: i32::MAX}; maze[0].len()]; maze.len()];
	for y in 1..maze.len() - 1 {
		for x in 1..maze[0].len()-1 {
			let up_position = Vector2D {x: x, y: y} + UP.as_usizes();
			let left_position = Vector2D {x: x, y: y} + LEFT.as_usizes();
			let right_position = Vector2D {x: x, y: y} + RIGHT.as_usizes();
			let down_position = Vector2D {x: x, y: y} + DOWN.as_usizes();
			if maze[up_position.y][up_position.x] !=  WALL_CHAR {
				maze_nodes[y][x].up = true;
			}
			if maze[left_position.y][left_position.x] != WALL_CHAR {
				maze_nodes[y][x].left = true;
			}
			if maze[right_position.y][right_position.x] != WALL_CHAR {
				maze_nodes[y][x].right = true;
			}
			if maze[down_position.y][down_position.x] != WALL_CHAR {
				maze_nodes[y][x].down = true;
			}
		}
	}

	update_nodes(&start, 0, RIGHT, &mut maze_nodes);
	print_matrix(maze);
	print_cost_matrix(&maze_nodes);

	println!("{:?}, {:?}", start, end);
	0
}

pub fn part_2(maze: &Vec<Vec<char>>) -> i32 {
	0
}
