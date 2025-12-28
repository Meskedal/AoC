use std::collections::{HashSet};

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

fn is_within(num: i64, a: i64, b: i64) -> bool {
	a <= num && num <= b
}

fn expand_ranges(candidate_id_ranges: &Vec<(i64, i64)>, mut all_id_ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
	let mut candidates_to_prune = vec![];
	for &(candidate_id_range_min, candidate_id_range_max) in candidate_id_ranges {
		let mut is_accounted_for = false;
		for (total_id_range_min, total_id_range_max) in all_id_ranges.iter_mut() {
			// If they are the exact same, no need to update anything. 
			if candidate_id_range_min == *total_id_range_min && candidate_id_range_max == *total_id_range_max {
				is_accounted_for = true;
				continue;
			}
			let min_within = is_within(candidate_id_range_min, *total_id_range_min, *total_id_range_max);
			let max_within = is_within(candidate_id_range_max, *total_id_range_min, *total_id_range_max);


			if min_within && max_within {
				// Redundant entry that should be removed if it exists in the final list. 
				candidates_to_prune.push((candidate_id_range_min, candidate_id_range_max));
				is_accounted_for = true;
				continue;
			}

			if !min_within && !max_within {
				continue;
			}

			if min_within {
				*total_id_range_max = candidate_id_range_max;
				is_accounted_for = true;

			} else if max_within {
				*total_id_range_min = candidate_id_range_min;
				is_accounted_for = true;
			}
		}
		if !is_accounted_for {
			all_id_ranges.push((candidate_id_range_min, candidate_id_range_max));
		}

	}

	// removing redundant entries that might have been found. 
	let mut all_id_ranges_pruned = vec![];
	for id_range in all_id_ranges {
		if !candidates_to_prune.contains(&id_range) {
			all_id_ranges_pruned.push(id_range);
		}
	}
	all_id_ranges_pruned
}

pub fn part_2(input: &(Vec<(i64, i64)>, Vec<i64>)) -> i64 {
	let (candidate_id_ranges, _) = input;
	let mut all_id_ranges: Vec<(i64, i64)> = candidate_id_ranges.clone();
	loop {
		let all_id_ranges_new  = expand_ranges(&all_id_ranges, all_id_ranges.clone());
		if all_id_ranges_new == all_id_ranges {
			break;
		}
		all_id_ranges = all_id_ranges_new;
	}
	let mut sum = 0;
	let mut ranges_set = HashSet::new(); // To remove duplicates 
	for id_range in all_id_ranges {
		if ranges_set.contains(&id_range) {
			continue;
		}
		ranges_set.insert(id_range);
		let (range_min, range_max) = id_range;
		sum +=  range_max - range_min + 1
	}
	sum
}
