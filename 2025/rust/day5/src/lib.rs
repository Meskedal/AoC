pub fn input_generator(input: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    // Split the sections for the rules and the section for the sequences 
    let sections: Vec<&str> = input.split("\r\n\r\n").collect();

	let ingredient_id_ranges = sections[0].lines().map(|l| {
		let mut id_ranges = l.split("-").map(|n| n.parse().unwrap());
		(id_ranges.next().unwrap(), id_ranges.next().unwrap())
	}).collect();

	let ingredient_ids = sections[1].lines().map(|l| l.parse().unwrap()).collect();

	(ingredient_id_ranges, ingredient_ids)
}

pub fn part_1(input: &(Vec<(i64, i64)>, Vec<i64>)) -> i64 {
	let (ingredient_id_ranges, ingredient_ids) = input;
	let mut sum = 0;
	'id_loop: for ingredient_id in ingredient_ids {
		for (ingredient_id_range_min, ingredient_id_range_max) in ingredient_id_ranges {
			if ingredient_id_range_min <= ingredient_id && ingredient_id <= ingredient_id_range_max {
				sum += 1;
				continue 'id_loop;
			}
		}
	}
	sum
}

pub fn part_2(input: &(Vec<(i64, i64)>, Vec<i64>)) -> i64 {
	let (ingredient_id_ranges, _) = input;
	let mut total_id_ranges: Vec<(i64, i64)> = vec![];
	for &(ingredient_id_range_min, ingredient_id_range_max) in ingredient_id_ranges {
		let mut updated = false;
		for (total_id_range_min, total_id_range_max) in total_id_ranges.iter_mut() {

		}
		if !updated {
			// Create a new entry 
			total_id_ranges.push((ingredient_id_range_min, ingredient_id_range_max));
		}
	}
	0
}
