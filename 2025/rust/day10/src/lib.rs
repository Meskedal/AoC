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

pub fn part_1(input: &(Vec<i32>, Vec<Vec<Vec<i32>>>, Vec<Vec<i32>>)) -> i32 {
    let (indicator_lights, buttons, _) = input;
    let mut sum = 0;
    for i in 0..indicator_lights.len() {
        let buttons = &buttons[i];
        let indicator_light = indicator_lights[i];
        let result = indicator_light_bfs(0, indicator_light, buttons, 10).unwrap();
        sum += result;
    }
    sum
}

pub fn part_2(input: &(Vec<i32>, Vec<Vec<Vec<i32>>>, Vec<Vec<i32>>)) -> i32 {
	0
}
