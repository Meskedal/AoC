pub fn input_generator(input: &str) -> (Vec<Vec<i64>>, Vec<&str>) {
	let lines: Vec<&str> = input.lines().collect();
	let end_line_index = lines.len()-1;
	let operators = lines[end_line_index].split_whitespace().collect();
	let numbers = lines[..end_line_index].iter().map(|l| l.split_whitespace().map(|w| w.parse().unwrap()).collect()).collect();
	(numbers, operators)
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

pub fn part_2(input: &(Vec<Vec<i64>>, Vec<&str>)) -> i64 {
	0
}
