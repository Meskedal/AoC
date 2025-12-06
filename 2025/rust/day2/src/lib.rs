pub fn input_generator(input: &str) -> Vec<(i64, i64)> {
    input
        .split(',')
        .map(|range| {
            let mut parts = range.split('-');
            let start = parts.next().unwrap().parse().unwrap();
            let end = parts.next().unwrap().parse().unwrap();
            (start, end)
        })
        .collect()
}

pub fn part_1(input: &[(i64, i64)]) -> i64 {
    let mut sum = 0;
    for (start, end) in input {
        for i in *start..=*end {
            let i_str = i.to_string();
            if i_str.len() % 2 != 0 {
                continue;
            }
            let i_str_1 = &i_str[..i_str.len()/2];
            let i_str_2 = &i_str[i_str.len()/2..i_str.len()];
            if i_str_1 == i_str_2 {
                sum += i;
            }
        }
    }
    sum
}

pub fn part_2(input: &[(i64, i64)]) -> i64 {
    let mut sum = 0;
    for (start, end) in input {
        'id: for i in *start..=*end {
            let i_str = i.to_string();
            let i_chars: Vec<char> = i_str.chars().collect();
            'substr: for chars in 1..=i_str.len()/2 {
                let sub_str = &i_str[0..chars];
                for chunk in i_chars[sub_str.len()..].chunks(sub_str.len()) {
                    let chunk_str: String = chunk.iter().collect();
                    if sub_str != chunk_str {
                        continue 'substr;
                    }
                }
                sum += i;
                continue 'id
            }
        }
    }
    sum
}
