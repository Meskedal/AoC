use std::collections::HashSet;

pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect()
}

fn within_bounds(row: i32, col: i32, row_min: usize, row_max: usize, col_min: usize, col_max: usize) -> bool {
    row >= row_min as i32 && row <= row_max as i32 && col >= col_min as i32 && col <= col_max as i32
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}

fn crawl(input: &Vec<Vec<u32>>, row: usize, col: usize, trailhead_next: u32, trailhead_max: u32, unique_paths: &mut HashSet<Position>) -> u32 {
    let row_min = 0;
    let col_min = 0;
    let row_max = input.len() - 1;
    let col_max = input[0].len() - 1;

    let mut valid_trailheads = 0;
    for row_offset in [-1, 1].iter() {
        let row = row as i32 + row_offset;
        if !(within_bounds(row, col as i32, row_min, row_max, col_min, col_max)) {
            continue;
        }
        if input[row as usize][col] == trailhead_next {
            if trailhead_next == trailhead_max  && !unique_paths.contains(&Position { row: row as usize, col }) {
                valid_trailheads += 1;
                unique_paths.insert(Position { row: row as usize, col });
                continue;
            }
            valid_trailheads += crawl(input, row as usize, col, trailhead_next + 1, trailhead_max, unique_paths);
        }
    }

    for col_offset in [-1, 1].iter() {
        let col = col as i32 + col_offset;
        if !(within_bounds(row as i32, col, row_min, row_max, col_min, col_max)) {
            continue;
        }
        if input[row][col as usize] == trailhead_next {
            if trailhead_next == trailhead_max  && !unique_paths.contains(&Position { row: row, col: col as usize }) {
                valid_trailheads += 1;
                unique_paths.insert(Position { row: row, col: col as usize});
                continue;
            }
            valid_trailheads += crawl(input, row, col as usize, trailhead_next + 1, trailhead_max, unique_paths);
        }
    }
    valid_trailheads
}

fn crawl_2(input: &Vec<Vec<u32>>, row: usize, col: usize, trailhead_next: u32, trailhead_max: u32) -> u32 {
    let row_min = 0;
    let col_min = 0;
    let row_max = input.len() - 1;
    let col_max = input[0].len() - 1;

    let mut valid_trailheads = 0;
    for row_offset in [-1, 1].iter() {
        let row = row as i32 + row_offset;
        if !(within_bounds(row, col as i32, row_min, row_max, col_min, col_max)) {
            continue;
        }
        if input[row as usize][col] == trailhead_next {
            if trailhead_next == trailhead_max {
                valid_trailheads += 1;
                continue;
            }
            valid_trailheads += crawl_2(input, row as usize, col, trailhead_next + 1, trailhead_max);
        }
    }

    for col_offset in [-1, 1].iter() {
        let col = col as i32 + col_offset;
        if !(within_bounds(row as i32, col, row_min, row_max, col_min, col_max)) {
            continue;
        }
        if input[row][col as usize] == trailhead_next {
            if trailhead_next == trailhead_max{
                valid_trailheads += 1;
                continue;
            }
            valid_trailheads += crawl_2(input, row, col as usize, trailhead_next + 1, trailhead_max);
        }
    }
    valid_trailheads
}

pub fn part_1(input: &Vec<Vec<u32>>) -> i32 {
    let row_min = 0;
    let row_max = input.len() - 1;
    let col_min = 0;
    let col_max = input[0].len() - 1;
    let trailhead_start = 0;
    let trailhead_end = 9;
    let mut trailhead_count = 0;
    for row in row_min..=row_max {
        for col in col_min..=col_max {
            if input[row][col] != trailhead_start {
                continue;
            }
            let mut unique_paths = HashSet::new();
            let trailhead_count_iteration = crawl(input, row, col, trailhead_start + 1, trailhead_end, &mut unique_paths);
            // println!("trailhead_count_iteration: {}", trailhead_count_iteration);
            // println!("trailstart at row: {}, col: {}", row, col);
            trailhead_count += trailhead_count_iteration;
        }
    }
    trailhead_count as i32
}

pub fn part_2(input: &Vec<Vec<u32>>) -> i32 {
        let row_min = 0;
    let row_max = input.len() - 1;
    let col_min = 0;
    let col_max = input[0].len() - 1;
    let trailhead_start = 0;
    let trailhead_end = 9;
    let mut trailhead_count = 0;
    for row in row_min..=row_max {
        for col in col_min..=col_max {
            if input[row][col] != trailhead_start {
                continue;
            }
            let trailhead_count_iteration = crawl_2(input, row, col, trailhead_start + 1, trailhead_end);
            // println!("trailhead_count_iteration: {}", trailhead_count_iteration);
            // println!("trailstart at row: {}, col: {}", row, col);
            trailhead_count += trailhead_count_iteration;
        }
    }
    trailhead_count as i32
}