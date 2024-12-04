use std::collections::HashMap;

#[aoc(day1, part1)]
fn part_1(input: &str) -> i32 {
    let mut total_distance = 0;
    let mut left_columns: Vec<i32> = vec![];
    let mut right_columns: Vec<i32> = vec![];

    for line in input.lines() {
        let mut columns = line.split_whitespace();
        let left_column = columns.next().unwrap().parse::<i32>().unwrap();
        let right_column = columns.next().unwrap().parse::<i32>().unwrap();
        left_columns.push(left_column);
        right_columns.push(right_column);
    }

    left_columns.sort();
    right_columns.sort();

    for (left_column, right_column) in left_columns.iter().zip(right_columns.iter()) {
        total_distance += (left_column - right_column).abs();
    }

    total_distance
}

#[aoc(day1, part2)]
fn part_2(input: &str) -> i32 {
    let mut left_columns: Vec<i32> = vec![];
    let mut similarity_score = 0;
    let mut right_column_instances_map: HashMap<i32, i32> = HashMap::new();

    for line in input.lines() {
        let mut columns = line.split_whitespace();
        let left_column = columns.next().unwrap().parse::<i32>().unwrap();
        let right_column = columns.next().unwrap().parse::<i32>().unwrap();
        left_columns.push(left_column);
        *right_column_instances_map.entry(right_column).or_insert(0) += 1;
    }

    for left_column in left_columns.iter() {
        let right_column_instances = right_column_instances_map.get(left_column).unwrap_or(&0);
        similarity_score += left_column * right_column_instances;
    }

    similarity_score
}