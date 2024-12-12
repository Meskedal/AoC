pub fn input_generator(input: &str) -> Vec<u64> {
    input.split(" ").map(|x| x.parse().unwrap()).collect()
}


fn rule1(stone: u64) -> Option<u64> {
    if stone != 0 {
        return None;
    }
    Some(1)
}

fn rule2(stone: u64) -> Option<(u64, u64)> {
    let s = stone.to_string();
    if s.len() % 2 != 0 {
        return None
    }

    let split = s.split_at(s.len()/2);
    Some((split.0.parse().unwrap(), split.1.parse().unwrap()))
}

fn rule3(stone: u64) -> u64 {
    stone*2024
}

pub fn part_1(input: &Vec<u64>) -> u64 {
    let mut stones = input.clone();
    for _ in 1..=25 {
        let mut new_stones = Vec::new();
        for stone in stones {
            if let Some(new_stone) = rule1(stone) {
                new_stones.push(new_stone);
            } else if let Some((a, b)) = rule2(stone) {
                new_stones.push(a);
                new_stones.push(b);
            } else {
                new_stones.push(rule3(stone));
            }
        }
        stones = new_stones;
        // println!("Blink {}: {:?}", blink, stones);
    }

    stones.len() as u64
}


pub fn part_2(input: &Vec<u64>) -> u64 {
    let mut stones = input.clone();
    for blink in 1..=75 {
        println!("Blink {}", blink);
        let mut new_stones = Vec::new();
        for stone in stones {
            if let Some(new_stone) = rule1(stone) {
                new_stones.push(new_stone);
            } else if let Some((a, b)) = rule2(stone) {
                new_stones.push(a);
                new_stones.push(b);
            } else {
                new_stones.push(rule3(stone));
            }
        }
        stones = new_stones;
    }
    stones.len() as u64
}
