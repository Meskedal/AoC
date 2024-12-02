use std::collections::HashMap;
fn main() {
    let input = include_str!("../input/day01.txt");
    let mut left_columns: Vec<i32> = vec![];
    let mut right_columns: Vec<i32> = vec![];
    let mut right_column_instances_map: HashMap<i32, i32> = HashMap::new();

    for line in input.lines() {
        // Part one:
        let mut columns = line.split_whitespace();
        let left_column = columns.next().unwrap().parse::<i32>().unwrap();
        let right_column = columns.next().unwrap().parse::<i32>().unwrap();
        left_columns.push(left_column);
        right_columns.push(right_column);
        // Part two:
        *right_column_instances_map.entry(right_column).or_insert(0) += 1;
    }

    left_columns.sort();
    right_columns.sort();

    let mut total_distance = 0;
    let mut similarity_score = 0;

    for (left_column, right_column) in left_columns.iter().zip(right_columns.iter()) {
        // Part one:
        total_distance += (left_column - right_column).abs();
        // Part two:
        let right_column_instances = right_column_instances_map.get(left_column).unwrap_or(&0);
        similarity_score += left_column * right_column_instances;
    }

    println!("Total distance: {}, Similarity Score: {}", total_distance, similarity_score);
}
