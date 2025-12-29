use regex::Regex;
use std::collections::VecDeque;

pub fn input_generator(input: &str) -> (Vec<i32>, Vec<Vec<Vec<i32>>>, Vec<Vec<i32>>) {
    let re = Regex::new(r"^\[(.*?)\]\s*((?:\([^\)]*\)\s*)+)\{([^\}]*)\}").unwrap();
    let re_parens = Regex::new(r"\(([^\)]*)\)").unwrap();
    
    let parsed: Vec<_> = input
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            let brackets = &caps[1];
            let parens = &caps[2];
            let braces = &caps[3];
            
            let indicator_light = brackets
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .fold(0, |acc, (i, _)| acc | (1 << i as i32));
            
            let buttons_line: Vec<Vec<i32>> = re_parens
                .captures_iter(parens)
                .map(|caps_match| {
                    caps_match[1]
                        .split(',')
                        .map(|d| d.trim().parse().unwrap())
                        .collect()
                })
                .collect();
            
            let joltage_entry: Vec<i32> = braces
                .split(',')
                .map(|d| d.trim().parse().unwrap())
                .collect();
            
            (indicator_light, buttons_line, joltage_entry)
        })
        .collect();
    
    parsed.into_iter().fold(
        (vec![], vec![], vec![]),
        |(mut lights, mut buttons, mut joltage), (light, button, jolt)| {
            lights.push(light);
            buttons.push(button);
            joltage.push(jolt);
            (lights, buttons, joltage)
        },
    )
}

fn xor(bits: i32, button: &[i32]) -> i32 {
    button.iter().fold(bits, |acc, &b| acc ^ (1 << b))
}

fn indicator_light_bfs(start_bits: i32, target: i32, buttons: &[Vec<i32>], max_depth: i32) -> Option<i32> {
    // Store the current indicator light state and the depth in the queue
    let mut queue = VecDeque::new();
    // Store the indicator light states as we discover them.
    queue.push_back((start_bits, 0));

    while let Some((bits, depth)) = queue.pop_front() {
        if depth >= max_depth {
            continue;
        }

        for b in buttons {
            let next = xor(bits, b);
            let next_depth = depth + 1;

            if next == target {
                return Some(next_depth); // first hit == minimal depth
            }

            // add to end the next indicator light state. 
            queue.push_back((next, next_depth));
        }
    }
    None
}

fn joltage_level_bfs(target: &[i32], list_of_buttons: &[Vec<i32>], max_depth: i32) -> Option<i32> {
    // Store the current joltage level and the depth in the queue
    let mut queue = VecDeque::new();
    // Store the current joltage levels as we discover them.
    let mut joltage_levels: Vec<i32> = target.to_vec();
    joltage_levels.fill(0);
    queue.push_back((joltage_levels, 0));

    while let Some((joltage_levels, depth)) = queue.pop_front() {
        if depth >= max_depth {
            continue;
        }

        'buttons: for buttons in list_of_buttons {
            let mut next_joltage_levels = joltage_levels.clone();
            for b in buttons {
                next_joltage_levels[*b as usize] += 1;
                // If we are above target, no need to search in this direction anymore
                if next_joltage_levels[*b as usize] > target[*b as usize] {
                    continue 'buttons;
                }
            } 
            let next_depth = depth + 1;

            if next_joltage_levels == target {
                return Some(next_depth); // first hit == minimal depth
            }
            // add to end the next indicator light state. 
            queue.push_back((next_joltage_levels, next_depth));
        }
    }
    None
}

fn joltage_level_A_star(target: &[i32], list_of_buttons: &[Vec<i32>]) -> Option<i32> {
    // Store the current joltage level and the depth in the queue
    let mut queue = VecDeque::new();
    // Store the current joltage levels as we discover them.
    let mut joltage_levels: Vec<i32> = target.to_vec();
    joltage_levels.fill(0);
    queue.push_back((joltage_levels, 0));

    while let Some((joltage_levels, depth)) = queue.pop_front() {
        for buttons in list_of_buttons {
            let mut next_joltage_levels = joltage_levels.clone();
            for b in buttons {
                next_joltage_levels[*b as usize] += 1;
            } 
            let next_depth = depth + 1;

            if next_joltage_levels == target {
                return Some(next_depth); // first hit == minimal depth
            }
            // add to end the next indicator light state. 
            queue.push_back((next_joltage_levels, next_depth));
        }
    }
    None
}

pub fn part_1(input: &(Vec<i32>, Vec<Vec<Vec<i32>>>, Vec<Vec<i32>>)) -> i32 {
    let (indicator_lights, buttons, _) = input;
    indicator_lights.iter().zip(buttons.iter()).map(|(il, b)| indicator_light_bfs(0, *il, b, 10).unwrap()).sum()
}

pub fn part_2(input: &(Vec<i32>, Vec<Vec<Vec<i32>>>, Vec<Vec<i32>>)) -> i32 {
    let (_, buttons, joltage) = input;
    let mut index = 0;
    joltage.iter().zip(buttons.iter()).map(|(joltage, b)| {
        index += 1;
        println!("{:?}", index);
        joltage_level_bfs(joltage, b, 20).unwrap()
    }).sum()
}
