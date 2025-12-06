pub fn input_generator(input: &str) -> Vec<String> {
	input.to_string().lines().map(|s| s.to_string()).collect()
}

pub fn part_1(input: &Vec<String>) -> i32 {
	let rotation: Vec<char> = input.iter().map(|s| s.chars().next().unwrap()).collect();
	let numbers: Vec<i32> = input.iter().map(|s| &s[1..])
		.map(|s| s.parse::<i32>().unwrap())
		.collect();
	let mut dial = 50;
	let mut count = 0;
	for i in 0..input.len() {
		let sign = match rotation[i] {
			'R' => 1,
			'L' => -1,
			_ => panic!("Invalid rotation character"),
		};
		dial += sign * numbers[i];
		dial = dial % 100;
		if dial == 0 {
			count += 1;
		}
	}
	count
}

pub fn part_2(input: &Vec<String>) -> i32 {
	let rotation: Vec<char> = input.iter().map(|s| s.chars().next().unwrap()).collect();
	let numbers: Vec<i32> = input.iter().map(|s| &s[1..])
		.map(|s| s.parse::<i32>().unwrap())
		.collect();
	let mut dial = 50;
	let mut count = 0;
	for i in 0..input.len() {
		let sign = match rotation[i] {
			'R' => 1,
			'L' => -1,
			_ => panic!("Invalid rotation character"),
		};
		let dial_before = dial;
		dial += sign * numbers[i];

		if dial <= 0 {
			let wrap_mult = (-dial / 100) + 1;
			dial = (dial + 100*wrap_mult) % 100;
			count += wrap_mult;
			if dial_before == 0 { // Did not cross zero from previous turn. 
				count -= 1;
			}
			// println!("Move {}{}: {} -> {}. Count: {}", rotation[i], numbers[i], dial_before, dial, (-dial/100 + 1).abs());
		} else if dial >= 100 {
			// println!("Move {}{}: {} -> {}. Count: {}", rotation[i], numbers[i], dial_before, dial, (dial/100).abs());
			count += dial / 100;
			dial = dial % 100;
		} 
	}
	count
}
