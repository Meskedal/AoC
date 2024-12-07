use std::collections::HashMap;

#[aoc_generator(day5)]
fn input_generator(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    // Split the sections for the rules and the section for the sequences 
    let sections: Vec<&str> = input.split("\r\n\r\n").collect();

    // The first section contains the page ordering rules
    let page_ordering_rules: Vec<(i32, i32)> = sections[0].lines().map(|l| {
        let mut nums = l.split("|").map(|n| n.trim().parse().unwrap());
        (nums.next().unwrap(), nums.next().unwrap())
    }).collect();

    // The second section contains the updates to check against the rules
    let updates: Vec<Vec<i32>> = sections[1].lines().map(|l| {
        l.split(",").map(|n| n.parse().unwrap()).collect()   
    }).collect();

    (page_ordering_rules, updates)
}

#[aoc(day5, part1)]
fn part_1((page_ordering_rules, updates): &(Vec<(i32, i32)>, Vec<Vec<i32>>)) -> i32 {
    let mut page_order_rules_map: HashMap<i32, Vec<i32>> = std::collections::HashMap::new();

    for &(page_order_rule, page_order_rulee) in page_ordering_rules.iter() {
        if let Some(rulee_page_order_rules) = page_order_rules_map.get_mut(&page_order_rulee) {
            rulee_page_order_rules.push(page_order_rule);
            continue;
        }
        page_order_rules_map.insert(page_order_rulee, vec![page_order_rule]);
    }
    let mut middle_page_number_sum = 0;
    for update in updates.iter() {
        let mut correctly_ordered_update = true;
        'page_loop: for (i, page_number) in update.iter().enumerate() {
            if let Some(rulee_rules) = page_order_rules_map.get(&page_number) {
                for forward_page_numbers in update.iter().skip(i + 1) {
                    if rulee_rules.contains(forward_page_numbers) {
                        correctly_ordered_update = false;
                        break 'page_loop;
                    }
                }
            }
        }

        if correctly_ordered_update {
            // Add middle element of sequence to the counter 
            middle_page_number_sum += update[update.len() / 2];
        }
    }
    middle_page_number_sum
}

#[aoc(day5, part2)]
fn part_2((page_ordering_rules, updates): &(Vec<(i32, i32)>, Vec<Vec<i32>>)) -> i32 {
    let mut page_order_rules_map: HashMap<i32, Vec<i32>> = std::collections::HashMap::new();
    let mut reverse_page_order_rules_map: HashMap<i32, Vec<i32>> = std::collections::HashMap::new();
    for &(page_order_rule, page_order_rulee) in page_ordering_rules.iter() {
        if let Some(rulee_page_order_rules) = page_order_rules_map.get_mut(&page_order_rulee) {
            rulee_page_order_rules.push(page_order_rule);
        } else {
            page_order_rules_map.insert(page_order_rulee, vec![page_order_rule]);
        }
        if let Some(rulee_page_order_rules) = reverse_page_order_rules_map.get_mut(&page_order_rule) {
            rulee_page_order_rules.push(page_order_rulee);
            continue;
        }
        reverse_page_order_rules_map.insert(page_order_rule, vec![page_order_rulee]);
    }
    let mut middle_page_number_sum = 0;
    for update in updates.iter() {
        let mut correctly_ordered_update = true;
        'page_loop: for (i, page_number) in update.iter().enumerate() {
            if let Some(rulee_rules) = page_order_rules_map.get(&page_number) {
                for forward_page_numbers in update.iter().skip(i + 1) {
                    if rulee_rules.contains(forward_page_numbers) {
                        correctly_ordered_update = false;
                        break 'page_loop;
                    }
                }
            }
        }

        // Don't care about correctly ordered updates in part 2
        if correctly_ordered_update {
            continue;
        }

        // n^2 for now, but can be optimized 
        let mut update_copy = update.clone(); 
        for i in 0..update_copy.len() {
            'page_loop: for j in 0..update_copy.len().saturating_sub(i) {
                let page_number = update_copy[j];
                if let Some(rulee_rules) = page_order_rules_map.get(&page_number) {
                    for k in j+1..update_copy.len().saturating_sub(i) {
                        let forward_page_number = update_copy[k];
                        if rulee_rules.contains(&forward_page_number) {
                            update_copy.swap(j, k);
                            continue 'page_loop;
                        }
                    }
                }
            }
        }
        // Add middle element of sequence to the counter 
        middle_page_number_sum += update_copy[update.len() / 2];
    }
    middle_page_number_sum
}