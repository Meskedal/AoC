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
	let rows_min = 0;
	let cols_min = 0;
	let rows_max = matrix.len();
	let cols_max = matrix[0].len();
	let mut num_neighbors = 0;
	for i in row.saturating_sub(1)..=row+1 {
		for j in col.saturating_sub(1)..=col+1 {
			if i == j {
				continue;
			}
			if i >= rows_max || j >= cols_max {
				continue;
			}
			if matrix[i][j] == '@' {
				num_neighbors += 1;
			}
		}
	}
	print_matrix(matrix);
	num_neighbors
}

pub fn part_1(input: &Vec<Vec<char>>) -> i32 {
	let rows_min = 0;
	let cols_min = 0;
	let rows_max = input.len();
	let cols_max = input[0].len();
	let mut sum = 0;
	let mut matrix = input.clone();
	for row in rows_min..rows_max {
		for col in cols_min..cols_max {
			if count_neighbors(input, row, col) < 4 {
				sum += 1;
				matrix[row][col] = 'X';
			}
		}
	}
	sum
}

pub fn part_2(input: &Vec<Vec<char>>) -> i32 {
	0
}
