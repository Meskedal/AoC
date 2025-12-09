use std::collections::HashSet;


pub fn input_generator(input: &str) -> Vec<Vec<char>> {
	input.lines().map(|l| l.chars().collect()).collect()
}

pub fn part_1(input: &Vec<Vec<char>>) -> i32 {
	let rows_max = input.len();
	let cols_max = input[0].len();

	let mut matrix = input.clone();
	let mut splitters = HashSet::new();
	for row in 0..rows_max-1 {
		for col in 0..cols_max {
			let c = matrix[row][col];
			if c != 'S' && c != '|' {
				continue;
			}
			let c_below = matrix[row+1][col];
			if c_below != '^' {
				matrix[row+1][col] = '|';
				continue;	
			}
			// Could alternatively just overwrite the splitter at this point
			splitters.insert((row+1, col));
			if col+1 < cols_max {
				matrix[row+1][col+1] = '|';
			}
			if col-1 > 0 {
				matrix[row+1][col-1] = '|';
			}
		}
	}
	splitters.len() as i32
}

fn print_matrix(matrix: &Vec<Vec<i64>>) {
	let rows_min = 0;
	let cols_min = 0;
	let rows_max = matrix.len();
	let cols_max = matrix[0].len();
	for row in rows_min..rows_max {
		for col in cols_min..cols_max {
			print!("{:>3}", matrix[row][col]);
		}
		println!("")
	}
}

pub fn part_2(input: &Vec<Vec<char>>) -> i64 {
	let rows_max = input.len();
	let cols_max = input[0].len();

	let mut quantum_path_count_mask: Vec<Vec<i64>> = vec![vec![0; cols_max]; rows_max];

	// Find start and update first quantum path
	for col in 0..cols_max {
		if input[0][col] == 'S' {
			quantum_path_count_mask[1][col] += 1;
			break;
		}
	}
	for row in 1..rows_max-1 {
		for col in 0..cols_max {
			let q = quantum_path_count_mask[row][col];
			if q == 0 { // No quantum action here
				continue;
			}
			let c_below = input[row+1][col];
			if c_below != '^' {
				quantum_path_count_mask[row+1][col] += q;
				continue;	
			}
			if col+1 < cols_max {
				quantum_path_count_mask[row+1][col+1] += q;
			}
			if col as isize -1 >= 0 {
				quantum_path_count_mask[row+1][col-1] += q;
			}
		}
	}
	// print_matrix(&quantum_matrix_mask);
	// Last row entries are the sum of all paths taken there.  
	quantum_path_count_mask[rows_max-1].iter().sum() 
}
