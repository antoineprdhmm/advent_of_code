use std::collections::{HashMap, HashSet};

use crate::read_input;

struct Machine {
    indicator_lights: Vec<char>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

fn read_machines() -> Vec<Machine> {
    read_input(2025, 10)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| {
            let parts: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

            let indicator_lights = parts[0]
                .trim_matches(|c| c == '[' || c == ']')
                .chars()
                .collect();

            let buttons = parts[1..(parts.len() - 1)]
                .iter()
                .map(|button_str| {
                    button_str
                        .trim_matches(|c| c == '(' || c == ')')
                        .split(',')
                        .map(|v| v.parse::<usize>().unwrap())
                        .collect()
                })
                .collect();

            let joltages = parts[parts.len() - 1]
                .trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .map(|v| v.parse::<usize>().unwrap())
                .collect();

            Machine {
                indicator_lights,
                buttons,
                joltages,
            }
        })
        .collect()
}

fn generate_all_combinations<T: Clone>(
    items: &[T],
    start: usize,
    cur: &mut Vec<T>,
    out: &mut Vec<Vec<T>>,
) {
    if !cur.is_empty() {
        out.push(cur.clone());
    }

    for i in start..items.len() {
        cur.push(items[i].clone());
        generate_all_combinations(items, i + 1, cur, out);
        cur.pop();
    }
}

fn indicator_lights_to_bits(indicator_lights: &[char]) -> u32 {
    indicator_lights.iter().fold(0, |acc, c| {
        (acc << 1)
            | match c {
                '#' => 1,
                '.' => 0,
                _ => panic!("Unexpected indicator light"),
            }
    })
}

fn button_to_bits(button: &[usize], indicator_lights_size: usize) -> u32 {
    let button_set: HashSet<_> = button.iter().collect();

    (0..indicator_lights_size).fold(0, |bits, i| {
        (bits << 1) | if button_set.contains(&i) { 1 } else { 0 }
    })
}

fn find_fewest_presses_to_configure_indicator_light(machine: &Machine) -> usize {
    let indicator_lights_bits = indicator_lights_to_bits(&machine.indicator_lights);
    let buttons_bits: Vec<u32> = machine
        .buttons
        .iter()
        .map(|b| button_to_bits(&b, machine.indicator_lights.len()))
        .collect();

    if buttons_bits.contains(&indicator_lights_bits) {
        return 1;
    }

    let mut all_combinations = Vec::new();
    generate_all_combinations(&buttons_bits, 0, &mut Vec::new(), &mut all_combinations);

    let mut combinations_by_len: HashMap<usize, Vec<Vec<u32>>> = HashMap::new();
    for combination in all_combinations {
        combinations_by_len
            .entry(combination.len())
            .or_default()
            .push(combination);
    }

    for nb_buttons in 2..*combinations_by_len.keys().max().unwrap() {
        for combination in combinations_by_len.get(&nb_buttons).unwrap() {
            let result_indicator_lights_bits = combination.iter().fold(0, |acc, x| acc ^ x);
            if result_indicator_lights_bits == indicator_lights_bits {
                return nb_buttons;
            }
        }
    }

    0
}

pub fn run_part_1() {
    let machines = read_machines();

    let total_fewest_presses: usize = machines
        .iter()
        .map(find_fewest_presses_to_configure_indicator_light)
        .sum();

    assert_eq!(total_fewest_presses, 432);
}

fn generate_pattern_costs(all_combinations: Vec<Vec<Vec<usize>>>) -> Vec<(Vec<usize>, usize)> {
    all_combinations
        .iter()
        .map(|combination| {
            let mut merged_masks = combination[0].clone();

            for i in 1..combination.len() {
                for j in 0..combination[i].len() {
                    merged_masks[j] += combination[i][j];
                }
            }

            // merge the masks of this combination
            // and associate the cost to it
            (merged_masks, combination.len())
        })
        .collect()
}

fn is_pattern_in_bounds(pattern: &[usize], target: &[usize]) -> bool {
    pattern.iter().zip(target).all(|(p, t)| p <= t)
}

fn is_same_parity(pattern: &[usize], target: &[usize]) -> bool {
    pattern.iter().zip(target).all(|(p, t)| p % 2 == t % 2)
}

fn find_fewest_presses_to_configure_joltages(
    target: Vec<usize>,
    pattern_costs: &Vec<(Vec<usize>, usize)>,
    cache: &mut HashMap<Vec<usize>, usize>,
) -> usize {
    if let Some(cached) = cache.get(&target) {
        return *cached;
    }

    // If all 0, no more press needed -> return 0
    if target.iter().sum::<usize>() == 0 {
        return 0;
    }

    // Arbitrary value, just big enought to be replaced
    let mut answer = 1000000;

    for (pattern, cost) in pattern_costs {
        // Key idea: if we can reach [1, 2, 3] joltage with n presses
        // Then we can reach [2, 4, 6] with 2*n presses
        // Just keep simplifying the problem
        if is_pattern_in_bounds(pattern, &target) && is_same_parity(pattern, &target) {
            let new_goal: Vec<usize> = pattern
                .iter()
                .zip(&target)
                .map(|(p, t)| (t - p) / 2)
                .collect();

            answer = answer.min(
                cost + 2 * find_fewest_presses_to_configure_joltages(
                    new_goal,
                    pattern_costs,
                    cache,
                ),
            );
        }
    }

    cache.insert(target, answer);
    answer
}

pub fn run_part_2() {
    let mut answer = 0;

    let machines = read_machines();

    for machine in machines {
        // Transform button like (1, 3) to mask like 0101
        // -> to know which impact it has on joltage
        let button_masks: Vec<Vec<usize>> = machine
            .buttons
            .iter()
            .map(|btn| {
                (0..machine.joltages.len())
                    .map(|i| if btn.contains(&i) { 1 } else { 0 })
                    .collect()
            })
            .collect();

        let mut all_combinations = Vec::new();
        generate_all_combinations(&button_masks, 0, &mut Vec::new(), &mut all_combinations);

        let mut pattern_costs = generate_pattern_costs(all_combinations);
        pattern_costs.push((vec![0; machine.joltages.len()], 0));

        let mut cache: HashMap<Vec<usize>, usize> = HashMap::new();

        answer +=
            find_fewest_presses_to_configure_joltages(machine.joltages, &pattern_costs, &mut cache);
    }

    assert_eq!(answer, 18011);
}
