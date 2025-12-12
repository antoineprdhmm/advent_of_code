use std::collections::HashMap;

use crate::read_input;

fn read_patterns_and_designs() -> (Vec<String>, Vec<String>) {
    let lines: Vec<String> = read_input(2024, 19)
        .unwrap()
        .map_while(Result::ok)
        .collect();

    let patterns = lines[0].split(", ").map(|s| s.to_string()).collect();
    let designs = lines[2..].iter().map(|s| s.to_string()).collect();

    (patterns, designs)
}

fn get_is_design_possible(i: usize, design: &str, patterns: &Vec<String>) -> bool {
    if i >= design.len() {
        return true;
    }

    'pattern_loop: for pattern in patterns {
        let mut j = i;
        for c in pattern.chars() {
            if j >= design.len() {
                continue 'pattern_loop;
            }
            if design.as_bytes()[j] != c as u8 {
                continue 'pattern_loop;
            }

            j += 1;
        }

        if get_is_design_possible(j, design, patterns) {
            return true;
        }
    }

    false
}

fn count_all_possible_combinations(
    i: usize,
    design: &str,
    patterns: &Vec<String>,
    memo: &mut HashMap<(usize, String), usize>,
) -> usize {
    if let Some(r) = memo.get(&(i, design.to_string())) {
        return *r;
    }

    let mut count = 0;

    if i >= design.len() {
        count = 1;
    } else {
        'pattern_loop: for pattern in patterns {
            if pattern.len() > (design.len() - i) {
                continue 'pattern_loop;
            }

            let mut j = i;
            for c in pattern.chars() {
                if design.as_bytes()[j] != c as u8 {
                    continue 'pattern_loop;
                }

                j += 1;
            }

            count += count_all_possible_combinations(j, design, patterns, memo);
        }
    }

    memo.insert((i, design.to_string()), count);

    count
}

pub fn run_part_1() {
    let (patterns, designs) = read_patterns_and_designs();

    let possible_designs = designs
        .iter()
        .filter(|design| get_is_design_possible(0, design, &patterns))
        .count();

    println!("{possible_designs}");
}

pub fn run_part_2() {
    let (patterns, designs) = read_patterns_and_designs();

    let mut memo: HashMap<(usize, String), usize> = HashMap::new();

    let possible_combinations_count = designs
        .iter()
        .map(|design| count_all_possible_combinations(0, design, &patterns, &mut memo))
        .sum::<usize>();

    println!("{possible_combinations_count}");
}
