pub fn input_generator(input: &str) -> (Vec<Vec<i64>>, Vec<&str>) {
	let lines: Vec<&str> = input.lines().collect();
	let end_line_index = lines.len()-1;
	let operators = lines[end_line_index].split_whitespace().collect();
	let numbers = lines[..end_line_index].iter().map(|l| l.split_whitespace().map(|w| w.parse().unwrap()).collect()).collect();
	(numbers, operators)
}

pub fn input_generator2(input: &str) -> Vec<Vec<char>> {
	input.lines().map(|l| l.chars().collect()).collect()
}

fn add(a: i64, b: i64) -> i64 {
	a + b
}

fn mul(a: i64, b: i64) -> i64 {
	a * b
}

pub fn part_1(input: &(Vec<Vec<i64>>, Vec<&str>)) -> i64 {
	let (numbers, operators) = input;
	let mut sum = 0;
	for col in 0..numbers[0].len() {
		let mut col_result = numbers[0][col];
		for row in 1..numbers.len() {
			col_result = match operators[col] {
				"*" => mul(col_result, numbers[row][col]),
				"+" => add(col_result, numbers[row][col]),
				_ => panic!()
			};
		}
		sum += col_result;
	}
	sum	
}



pub fn part_2(input: &Vec<Vec<char>>) -> i64 {
	let mut start_col = 0;
	let mut end_col = 0;

	let rows_max = input.len();
	let cols_max = input[0].len();
	let operator_row = rows_max - 1;
	let mut sum = 0;
	while start_col != cols_max {
		end_col = cols_max; // set to max col, unless a operator is found underneath
		for i in start_col+1..cols_max {
			if !input[operator_row][i].is_whitespace() {
				end_col = i;
				break;
			}
		}
		let operator = input[operator_row][start_col];
		let mut result_value = if operator == '*' {1} else {0};
		for col in start_col..end_col {
			let mut col_num_str = String::new();
			for row in 0..rows_max - 1 {
				let c = input[row][col];
				if c.is_numeric() {
					col_num_str.push(input[row][col]);
				}
			}
			if col_num_str.is_empty() {
				continue;
			}
			let col_num = col_num_str.parse().unwrap();
			result_value = match operator {
				'*' => mul(result_value, col_num),
				'+' => add(result_value, col_num),
				_ => panic!()
			};
		}
		sum += result_value;	
		start_col = end_col
	}
	sum
}
