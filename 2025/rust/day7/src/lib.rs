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

pub fn part_2(input: &Vec<Vec<char>>) -> i32 {
	0
}
