use std::cmp;

pub fn input_generator(input: &str) -> Vec<(i64, i64)> {
	input.lines().map(|l| {
		let mut nums = l.split(',');
		(nums.next().unwrap().parse().unwrap(), nums.next().unwrap().parse().unwrap())
	}).collect()
}

fn area(a: (i64, i64), b:(i64, i64)) -> i64 {
	((b.0 - a.0).abs()+1)*((b.1 - a.1).abs()+1)
}

pub fn part_1(input: &Vec<(i64, i64)>) -> i64 {
	input.iter().enumerate().flat_map(|(i, &a)| {
		input.iter().skip(i + 1).map(move |&b| area(a, b))
	}).max().unwrap()
}

pub fn part_2(red_tiles: &Vec<(i64, i64)>) -> i64 {
	let mut largest_rectangle_area = 0;
    for i in 0..red_tiles.len()-1 {
        'loop_j: for j in i+1..red_tiles.len() {
			let v1 = red_tiles[i];
			let v3 = red_tiles[j];
			let v2 = (v1.0, v3.1);
			let v4 = (v3.0, v1.1);
			let rectangle_vertices = vec![v1,v2,v3,v4];
			let max_rect_x = rectangle_vertices.iter().map(|&v| v.0).max().unwrap();
			let min_rect_x = rectangle_vertices.iter().map(|&v| v.0).min().unwrap();
			let max_rect_y = rectangle_vertices.iter().map(|&v| v.1).max().unwrap();
			let min_rect_y = rectangle_vertices.iter().map(|&v| v.1).min().unwrap();
			for (i, &rs1) in rectangle_vertices.iter().enumerate() {
				let rs2 = rectangle_vertices[(i+1)%rectangle_vertices.len()];
				let max_rs_x = cmp::max(rs1.0, rs2.0);
				let min_rs_x = cmp::min(rs1.0, rs2.0);
				let max_rs_y = cmp::max(rs1.1, rs2.1);
				let min_rs_y = cmp::min( rs1.1, rs2.1);
				// For each side of the rectangle, check if it goes through a polygon side 
				// Check if the rectangle side is a horizontal or vertical side 
				let horizontal_rectangle_side = rs1.1 == rs2.1;
				for k in 0..red_tiles.len() {
					let ps1 = red_tiles[k];
					let ps2 = red_tiles[(k+1)%red_tiles.len()];

					if (min_rect_x < ps1.0 && ps1.0 < max_rect_x) && (min_rect_y < ps1.1 && ps1.1 < max_rect_y) {
						// Do not allow points within the rectangle, could theoretically be possible, but not in this input.
						continue 'loop_j;
					}

					let horizontal_polygon_side = ps1.1 == ps2.1;
					let max_ps_x = cmp::max(ps1.0,ps2.0);
					let min_ps_x = cmp::min(ps1.0,ps2.0);
					let max_ps_y = cmp::max(ps1.1,ps2.1);
					let min_ps_y = cmp::min(ps1.1,ps2.1);
					match (horizontal_rectangle_side, horizontal_polygon_side) {
						(true, true) | (false, false) => {continue},
						(true, false) => {
							// Horizontal rectangle side, vertical polygon side
							if min_rs_x <= ps1.0 && ps1.0 <= max_rs_x { // Polygon x is inside rectangle x range
								if min_ps_y < rs1.1 && rs1.1 < max_ps_y { // rectangle y is inside polygon y range
									continue 'loop_j;
								}
							}
						}
						(false, true) => {
							// Vertical rectangle side, horizontal polygon side
							if min_ps_x <= rs1.0 && rs1.0 <= max_ps_x { // Rectangle x is inside polygon x range
								if min_rs_y < ps1.1 && ps1.1 < max_rs_y { // Polygon y in inside rectangle y range
									continue 'loop_j;
								}
							}
						}
					}
				}
			}
            let area = area(v1, v3);
            if area > largest_rectangle_area {
                largest_rectangle_area = area;
            }
        }
    }
	largest_rectangle_area
}