use regex::Regex;

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

pub fn part_1(input: &(Vec<i32>, Vec<Vec<Vec<i32>>>, Vec<Vec<i32>>)) -> i32 {
	let (indicator_lights, buttons, joltage) = input;
	for i in 0..indicator_lights.len() {
		let buttons = &buttons[i];
		let indicator_light = indicator_lights[i];
		// Now the actual part 1
	}
	0
}

pub fn part_2(input: &(Vec<i32>, Vec<Vec<Vec<i32>>>, Vec<Vec<i32>>)) -> i32 {
	0
}
