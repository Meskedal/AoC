pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    let test = input.lines().map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect()).collect();
    test
}

fn safe_predicate_1(x0: i32, x1: i32, x2: i32) -> bool {
    // TODO: Identify the slopes by using the diff between two values instead. 
    if x0 > x1 { // Initially decreasing slope
        if !(x1 > x2) { // Then not decreasing slope
            return false;
        }
    } else if x0 < x1 { // Initially increasing slope
        if !(x1 < x2) { // Then not increasing slope
            return false;
        }
    } else { // Flat slope
        return false;
    }
    true
}

fn safe_predicate_2(x0: i32, x1: i32, x2: i32) -> bool {
    let diff1 = (x1 - x0).abs(); 
    let diff2 = (x1 - x2).abs();
    if diff1 < 1 || diff1 > 3 || diff2 < 1 || diff2 > 3 {
        return false;
    }
    true
}

fn safe_predicate_part_1(levels: &Vec<i32>) -> bool {
    for i in 1..levels.len()-1 {
        if !safe_predicate_1(levels[i-1], levels[i], levels[i+1]) {
            return false;
        }

        if !safe_predicate_2(levels[i-1], levels[i], levels[i+1]) {
            return false;
        }
    }
    true
}

fn safe_predicate_part_2(levels: &Vec<i32>) -> bool {
    let mut safe = true;
    for i in 1..levels.len()-1 {
        if !safe_predicate_1(levels[i-1], levels[i], levels[i+1]) {
            safe = false;
            break;
        }

        if !safe_predicate_2(levels[i-1], levels[i], levels[i+1]) {
            safe = false;
            break;
        }
    }

    // If the levels are deemed safe at this point, return true
    if safe {
        return true;
    }

    // Evaluate if removing one level makes the report/levels safe
    for i in 0..levels.len() {
        safe = true;
        // TODO: Avoid copying the levels by allocating fixed size array and copying the values
        let new_levels: Vec<i32> = levels.iter().copied().enumerate().filter(|(j, _)| *j != i).map(|(_, x)| x).collect();
        for j in 1..new_levels.len()-1 {
            if !safe_predicate_1(new_levels[j-1], new_levels[j], new_levels[j+1]) {
                safe = false;
                break;
            }

            if !safe_predicate_2(new_levels[j-1], new_levels[j], new_levels[j+1]) {
                safe = false;
                break;
            }
        }
        if safe {
            return true;
        }
    }
    false
}


pub fn part_1(input: &Vec<Vec<i32>>) -> i32 {
    let mut safe_reports = 0;
    for levels in input.iter() {
        if !safe_predicate_part_1(levels) {
            continue;
        }
        safe_reports += 1;
    }
    safe_reports
}

pub fn part_2(input: &Vec<Vec<i32>>) -> i32 {
    let mut safe_reports = 0;

    for levels in input.iter() {
        if !safe_predicate_part_2(levels) {
            continue;
        }
        safe_reports += 1;
    }
    safe_reports
}
