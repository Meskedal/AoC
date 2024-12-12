use std::ops::AddAssign;

pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Region {
    perimeter_count: i32, 
    area_count: i32, 
}

impl AddAssign for Region {
    fn add_assign(&mut self, other: Self) {
        self.perimeter_count += other.perimeter_count;
        self.area_count += other.area_count;
    }
}

fn within_bounds(row: i32, col: i32, chars: &Vec<Vec<char>>) -> bool {
    col >= 0 && col < chars[0].len() as i32 && row >= 0 && row < chars.len() as i32
}

fn visit(crop: char, chars: &Vec<Vec<char>>, row: i32, col: i32, visited: &mut Vec<Vec<bool>>) -> Region {
    let mut region = Region { perimeter_count: 0, area_count: 0 };
    if !within_bounds(row, col, &chars) || visited[row as usize][col as usize] {
        return region;
    }
    if chars[row as usize][col as usize] != crop {
        return region;
    }
    visited[row as usize][col as usize] = true;
    region.area_count = 1;

    for [drow, dcol] in [[-1, 0], [1, 0], [0, -1], [0, 1]] {
        if !within_bounds(row + drow, col + dcol, chars) {
            region.perimeter_count += 1;
        } else if chars[(row as i32 + drow) as usize][(col as i32 + dcol) as usize] != crop {
            region.perimeter_count += 1;
        } else {
            region += visit(crop, chars, row + drow, col + dcol, visited);
        }
    }
    region
}

pub fn part_1(chars: &Vec<Vec<char>>) -> i32 {
    let mut visited = vec![vec![false; chars[0].len()]; chars.len()];
    let mut sum = 0;
    for row in 0..chars.len() {
        for col in 0..chars[row].len() {
            if visited[row][col] {
                continue;
            }
            let region = visit(chars[row][col], &chars, row as i32, col as i32, &mut visited);
            sum += region.area_count * region.perimeter_count;
        }
    }
    sum
}


pub fn part_2(input: &Vec<Vec<char>>) -> i32 {
    0
}
