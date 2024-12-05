use std::collections::HashMap;

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<Vec<char>> {
    // Create a character matrix based on each row and column of the input string 
    let char_matrix = input.lines().map(|l| l.chars().collect()).collect();
    char_matrix
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vector {
    row: i32,
    col: i32,
}

#[aoc(day4, part1)]
fn part_1(char_matrix: &Vec<Vec<char>>) -> i32 {
    let rows_min: i32 = 0;
    let cols_min: i32 = 0;
    let rows_max = char_matrix.len() as i32;
    let cols_max = char_matrix[0].len() as i32;
    let word = "XMAS";
    let char_first = word.chars().next().unwrap();
    let mut word_count = 0;

    let mut unit_directions: Vec<Vector> = vec![];
    // Create a list of all possible unit directions
    for i in [-1, 0, 1].iter() {
        for j in [-1, 0, 1].iter() {
            // Skip the case where both i and j are 0
            if *i == 0 && *j == 0 {
                continue;
            }
            unit_directions.push(Vector { row: *i, col: *j });
        }
    }
    for i in 0..rows_max {
        for j in 0..cols_max {
            // Determine which word to search for 
            if char_matrix[i as usize][j as usize] != char_first {
                continue;
            } 

            // We have found the start of the word of interest, search in all directions for the rest of the word. 
            for unit_direction in &unit_directions {
                let mut found = true;
                for letter_index in  1..word.len() {
                    // Increment the row and column by the unit direction and the letter index
                    let row = i as i32 + unit_direction.row*letter_index as i32;
                    let col = j as i32 + unit_direction.col*letter_index as i32;
                    // Check if we are still within the bounds of the matrix 
                    if !(row >= rows_min && row < rows_max && col >= cols_min && col < cols_max) {
                        found = false;
                        break;
                    }
                    // Check if we are still tracking the word 
                    if char_matrix[row as usize][col as usize] != word.chars().nth(letter_index).unwrap() {
                        found = false;
                        break;
                    }
                }

                if found {
                    word_count += 1;
                }
            }
        }
    }
    word_count
}

#[aoc(day4, part2)]
fn part_2(char_matrix: &Vec<Vec<char>>) -> i32 {
    // let rows_min: i32 = 1;
    // let cols_min: i32 = 1;
    let rows_max = char_matrix.len() as i32;
    let cols_max = char_matrix[0].len() as i32;

    let opposite_diagonal_letter_map = HashMap::from([
        ('M', 'S'),
        ('S', 'M'),
    ]);
    let middle_letter = 'A';
    let mut word_count = 0;

    // Define the diagonal directions, the opposite diagonal direction pair for each diagonal is found by taking the inverse of the sign. (i.e multiplying by -1)
    let diagonal_directions: Vec<Vector> = vec![Vector {row: 1, col: 1}, Vector {row: 1, col: -1}];
    for i in 1..rows_max-1 { // Always searching within the bounds of the matrix in this case. 
        for j in 1..cols_max-1 { // Always searching within the bounds of the matrix in this case.
            if char_matrix[i as usize][j as usize] != middle_letter {
                continue;
            } 

            // We have found the start of the cross, check the diagonals and the opposite diagonals and the corresponding opposite letters
            let mut found = true;
            for diagonal_direction in &diagonal_directions {
                let col = j as i32 + diagonal_direction.col;
                let row = i as i32 + diagonal_direction.row;
                let diag_letter = char_matrix[row as usize][col as usize];
                if let Some(&opposite_letter) = opposite_diagonal_letter_map.get(&diag_letter) {
                    let col_opposite = j as i32 - diagonal_direction.col;
                    let row_opposite = i as i32 - diagonal_direction.row;
                    if char_matrix[row_opposite as usize][col_opposite as usize] != opposite_letter {
                        found = false;
                        break;
                    }
                } else {
                    found = false;
                    break;
                }
            }
            if found {
                word_count += 1;
            }
        }
    }
    word_count
}