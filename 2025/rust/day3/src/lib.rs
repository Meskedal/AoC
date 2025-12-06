use core::panic;

pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
	input.lines().map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect()).collect()
}

pub fn part_1(input: &Vec<Vec<i32>>) -> i32 {
	let mut sum = 0;
	for bank in input {
		let mut largest_battery = bank[0];
		let mut largest_battery_index = 0;
		for (index, &battery) in bank[1..bank.len()-1].iter().enumerate() {
			if battery > largest_battery {
				largest_battery = battery;
				largest_battery_index = index+1;
			}
		}
		let mut second_largest_battery = bank[largest_battery_index+1];
		for &battery in bank[largest_battery_index+1+1..].iter() {
			if battery > second_largest_battery {
				second_largest_battery = battery;
			}
		}
		let concatenated: i32 = (largest_battery.to_string() + &second_largest_battery.to_string())
            .parse()
            .unwrap();
        sum += concatenated;

	}
	sum
}

fn find_next_battery(batteries: &Vec<i32>, start_index: usize, num_batteries_remaining: usize) -> Option<(usize, i32)> {
	if num_batteries_remaining == 0 {
		return None;
	}

	let end_index = batteries.len() - num_batteries_remaining;
	let mut largest_battery = -1;
	let mut largest_battery_index = 0;
	for (index, &battery) in batteries[start_index..=end_index].iter().enumerate() {
		if battery > largest_battery {
			largest_battery = battery;
			largest_battery_index = start_index + index;
		}
	}
	Some((largest_battery_index, largest_battery))
}

pub fn part_2(input: &Vec<Vec<i32>>) -> i64 {
	let mut sum = 0;
	for bank in input {
		let mut start_index = 0;
		let mut num_batteries_remaining = 12;
		let mut batteries_concatenated = String::new();

		while num_batteries_remaining > 0 {
			let next_battery = find_next_battery(bank, start_index, num_batteries_remaining);
			if let Some((index, battery)) = next_battery {
				start_index = index + 1;
				batteries_concatenated.push_str(&battery.to_string());
				num_batteries_remaining -= 1;
			} else {
				panic!();
			}
		}
		sum += batteries_concatenated.parse::<i64>().unwrap();	

	}
	sum
}
