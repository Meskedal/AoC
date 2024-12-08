use regex::Regex;

pub fn part_1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)+").unwrap();
    let captures = re.captures_iter(input);
    let mut multiplication_sum = 0;
    for i in captures {
        let mul_1 = i[1].parse::<i32>().unwrap();
        let mul_2 = i[2].parse::<i32>().unwrap();
        multiplication_sum += mul_1 * mul_2;
    }
    multiplication_sum
}

pub fn part_2(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)+").unwrap();
    let mut multiplication_sum = 0;
    let donts: Vec<&str> = input.split("don't").collect();

    // The initial string should be treated as a "do" string 
    let captures = re.captures_iter(donts[0]);
    for i in captures {
        let mul_1 = i[1].parse::<i32>().unwrap();
        let mul_2 = i[2].parse::<i32>().unwrap();
        multiplication_sum += mul_1 * mul_2;
    }

    for dont in donts[1..].iter() {
        let dos: Vec<&str> = dont.splitn(2, "do").collect();
        if dos.len() < 2 {
            continue;
        }
        // Cannot define variable as "do" is a reserved keyword
        let do_str = dos[1];
        let captures = re.captures_iter(do_str);
        for i in captures {
            let mul_1 = i[1].parse::<i32>().unwrap();
            let mul_2 = i[2].parse::<i32>().unwrap();
            multiplication_sum += mul_1 * mul_2;
        }
    }
    multiplication_sum
}