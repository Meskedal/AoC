use core::num;
use std::ops;
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Coordinate {
	x: i64,
	y: i64,
	z: i64,
}

impl Coordinate {
	fn length(&self) -> i64 {
		((self.x*self.x + self.y*self.y + self.z*self.z) as f64).sqrt() as i64
	}

	fn euclidean_distance(&self, other: &Coordinate) -> i64 {
		(self - other).length()
	}
}

impl ops::Sub<&Coordinate> for &Coordinate {
	type Output = Coordinate;
	fn sub(self, _rhs: &Coordinate) -> Coordinate {
		Coordinate { x: (self.x - _rhs.x), y: (self.y - _rhs.y), z: (self.z - _rhs.z) }
	}
}

pub fn input_generator(input: &str) -> Vec<Coordinate> {
	input.lines().map(|l| {
		let mut it = l.split(',');
		Coordinate { x: {it.next().unwrap().parse().unwrap()}, y: {it.next().unwrap().parse().unwrap()}, z: {it.next().unwrap().parse().unwrap()} }
	}).collect()
}

pub fn part_1(input: &Vec<Coordinate>, target_junction_pairs: usize) -> i64 {
	let mut junction_pair_lengths: Vec<(usize, usize, i64)> = vec![];
	let mut circuits: Vec<Vec<Coordinate>> = vec![];
	for i in 0..input.len()-1 {
		for j in i+1..input.len() {
			junction_pair_lengths.push((i, j, input[i].euclidean_distance(&input[j])));
		}
	}
	junction_pair_lengths.sort_by(|a, b| a.2.cmp(&b.2));

	junction_pair_lengths = input.iter().enumerate().flat_map(|(i, &a)| {
		input.iter().skip(i+1).enumerate().map(move |(j, &b)| (i,j,a.euclidean_distance(&b)))
	}).collect();

	let mut junction_pairs_count = 0;
	let mut junction_pair_index = 0;
	'outer: while junction_pairs_count < target_junction_pairs {
		// Check if the either exists in a junction already.
		let closest_junction_pair = junction_pair_lengths[junction_pair_index];
		junction_pair_index += 1;
		let mut junction_a_circuit = None;
		let mut junction_b_circuit = None;
		for i in 0..circuits.len() {
			let junction_a_present = circuits[i].iter().any(|c| c == &input[closest_junction_pair.0]);
			let junction_b_present = circuits[i].iter().any(|c| c == &input[closest_junction_pair.1]);

			if junction_a_present && junction_b_present {
				junction_pairs_count += 1;
				continue 'outer;
			}

			if junction_a_present {
				junction_a_circuit = Some(i);
			}

			if junction_b_present {
				junction_b_circuit = Some(i);
			}	

			match (junction_a_circuit, junction_b_circuit) {
				(Some(_a), Some(_b)) => {break},
				_ => {}
			}
		}
		if let (Some(junction_a_circuit), Some(junction_b_circuit)) = (junction_a_circuit, junction_b_circuit) {
			// We have to merge circuits, merge b into a 
			// Ensure we always remove the higher index first to avoid shifting indices
			let (a, b) = if junction_a_circuit > junction_b_circuit {
				(junction_a_circuit, junction_b_circuit)
			} else {
				(junction_b_circuit, junction_a_circuit)
			};
			let mut circuit_to_merge = circuits.remove(a);
			circuits[b].append(&mut circuit_to_merge);
		} else if let Some(junction_a_circuit) = junction_a_circuit {
			circuits[junction_a_circuit].push(input[closest_junction_pair.1]);
		} else if let Some(junction_b_circuit) = junction_b_circuit {
			circuits[junction_b_circuit].push(input[closest_junction_pair.0]);
		} else {
			circuits.push(vec![input[closest_junction_pair.0], input[closest_junction_pair.1]]);
		}
		junction_pairs_count += 1;
	}

	circuits.sort_by(|a, b| b.len().cmp(&a.len()));
	circuits.iter().take(3).map(|c| c.len() as i64).product()
}

pub fn part_2(input: &Vec<Coordinate>) -> i64 {
	let mut junction_pair_lengths: Vec<(usize, usize, i64)> = vec![];
	let mut circuits: Vec<Vec<Coordinate>> = vec![];
	for i in 0..input.len()-1 {
		for j in i+1..input.len() {
			junction_pair_lengths.push((i, j, input[i].euclidean_distance(&input[j])));
		}
	}
	junction_pair_lengths.sort_by(|a, b| a.2.cmp(&b.2));

	let mut junction_pair_index = 0;
	let mut last_closest_junction_par = junction_pair_lengths[0];
	// This time we loop until sum of circuit lengths are equal to the input length,
	// meaning all the junctions are present. 
	'outer: while circuits.iter().map(|c| c.len()).sum::<usize>() < input.len() {
		// Check if the either exists in a junction already.
		let closest_junction_pair = junction_pair_lengths[junction_pair_index];
		junction_pair_index += 1;
		let mut junction_a_circuit = None;
		let mut junction_b_circuit = None;
		for i in 0..circuits.len() {
			let junction_a_present = circuits[i].iter().any(|c| c == &input[closest_junction_pair.0]);
			let junction_b_present = circuits[i].iter().any(|c| c == &input[closest_junction_pair.1]);

			if junction_a_present && junction_b_present {
				continue 'outer;
			}

			if junction_a_present {
				junction_a_circuit = Some(i);
			}

			if junction_b_present {
				junction_b_circuit = Some(i);
			}	

			match (junction_a_circuit, junction_b_circuit) {
				(Some(_a), Some(_b)) => {break},
				_ => {}
			}
		}
		if let (Some(junction_a_circuit), Some(junction_b_circuit)) = (junction_a_circuit, junction_b_circuit) {
			// We have to merge circuits, merge b into a 
			// Ensure we always remove the higher index first to avoid shifting indices
			let (a, b) = if junction_a_circuit > junction_b_circuit {
				(junction_a_circuit, junction_b_circuit)
			} else {
				(junction_b_circuit, junction_a_circuit)
			};
			let mut circuit_to_merge = circuits.remove(a);
			circuits[b].append(&mut circuit_to_merge);
		} else if let Some(junction_a_circuit) = junction_a_circuit {
			circuits[junction_a_circuit].push(input[closest_junction_pair.1]);
		} else if let Some(junction_b_circuit) = junction_b_circuit {
			circuits[junction_b_circuit].push(input[closest_junction_pair.0]);
		} else {
			circuits.push(vec![input[closest_junction_pair.0], input[closest_junction_pair.1]]);
		}
		last_closest_junction_par = closest_junction_pair;
	}
	input[last_closest_junction_par.0].x * input[last_closest_junction_par.1].x
}
