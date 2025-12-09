pub fn input_generator(input: &str) -> Vec<Vec<char>> {
	input.lines().map(|l| l.chars().collect()).collect()
}

fn print_matrix(matrix: &Vec<Vec<char>>) {
	let rows_min = 0;
	let cols_min = 0;
	let rows_max = matrix.len();
	let cols_max = matrix[0].len();
	for row in rows_min..rows_max {
		for col in cols_min..cols_max {
			print!("{}", matrix[row][col]);
		}
		println!("")
	}
}

fn count_neighbors(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
	let rows_max = matrix.len();
	let cols_max = matrix[0].len();
	const OFFSETS: [(isize, isize); 8] = [
		(-1, -1), (-1, 0), (-1, 1),
		(0, -1),          (0, 1),
		(1, -1),  (1, 0),  (1, 1),
	];
	OFFSETS.iter().filter_map(|(dr, dc)| {
		let nr = row as isize + dr;
		let nc = col as isize + dc;
		if nr < 0 || nr >= rows_max as isize || nc < 0 || nc >= cols_max as isize {
			return None;
		}
		Some(matrix[nr as usize][nc as usize])
	}).filter(|&val| val == '@').count() as i32
}

pub fn part_1(input: &Vec<Vec<char>>) -> i32 {
	let rows_max = input.len();
	let cols_max = input[0].len();
	let mut sum = 0;
	let mut matrix_debug = input.clone();
	for row in 0..rows_max {
		for col in 0..cols_max {
			if matrix_debug[row][col] != '@' {
				continue;
			}
			if count_neighbors(input, row, col) < 4 {
				sum += 1;
				matrix_debug[row][col] = 'X';
			}
		}
	}
	print_matrix(&matrix_debug);
	sum
}

pub fn part_2(input: &Vec<Vec<char>>) -> i32 {
	let rows_max = input.len();
	let cols_max = input[0].len();
	let mut sum = 0;
	let mut matrix = input.clone();
	loop {
		// print_matrix(&matrix);
		let mut matrix_next = matrix.clone();
		let sum_before = sum;
		for row in 0..rows_max {
			for col in 0..cols_max {
				if matrix[row][col] != '@' {
					continue;
				}
				if count_neighbors(&matrix, row, col) < 4 {
					sum += 1;
					matrix_next[row][col] = '.';
				}
			}
		}
		if sum == sum_before {
			// No reduction was made, we are done. 
			break;
		}
		matrix = matrix_next;
	}
	sum
}
