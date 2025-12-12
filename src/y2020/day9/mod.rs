use std::collections::{hash_map::Entry, HashMap};

use crate::read_input;

fn read_numbers() -> Vec<usize> {
    read_input(2020, 9)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

fn check_is_sum_of_two_options(
    options_counts: &HashMap<usize, usize>,
    numbers: &[usize],
    i_number_to_validate: usize,
    options_size: usize,
) -> bool {
    let number_to_validate = numbers[i_number_to_validate];

    for number_to_subtract in numbers
        .iter()
        .take(i_number_to_validate)
        .skip(i_number_to_validate - options_size)
    {
        if number_to_validate > *number_to_subtract {
            let remaining = number_to_validate - number_to_subtract;

            if let Some(remaining_count) = options_counts.get(&remaining) {
                // if it's the same, then need at least 2 available
                if remaining == *number_to_subtract && remaining_count >= &2 {
                    return true;
                }
                if remaining != *number_to_subtract && remaining_count >= &1 {
                    return true;
                }
            }
        }
    }

    false
}

fn find_first_wrong_number(numbers: &[usize], options_size: usize) -> Option<usize> {
    let mut options_counts: HashMap<usize, usize> = HashMap::new();
    for number in numbers.iter().take(options_size) {
        *options_counts.entry(*number).or_insert(0) += 1;
    }

    for i_number_to_validate in options_size..numbers.len() {
        let is_sum_of_two_options = check_is_sum_of_two_options(
            &options_counts,
            numbers,
            i_number_to_validate,
            options_size,
        );

        if !is_sum_of_two_options {
            return Some(numbers[i_number_to_validate]);
        }

        // sliding window for options_counts
        // remove oldest option, and add new option

        let entry = options_counts.entry(numbers[i_number_to_validate]);

        if let Entry::Occupied(mut o) = entry {
            let count = o.get_mut();
            *count -= 1;
            if *count == 0 {
                o.remove();
            }
        }

        *options_counts
            .entry(numbers[i_number_to_validate])
            .or_insert(0) += 1;
    }

    None
}

pub fn run_part_1() {
    let numbers = read_numbers();
    let first_wrong_number = find_first_wrong_number(&numbers, 25);

    assert_eq!(first_wrong_number, Some(104054607));
}

fn find_encryption_weakness(numbers: &[usize], expected_sum: usize) -> usize {
    for set_size in 2..numbers.len() {
        let mut sum = numbers.iter().take(set_size).sum::<usize>();

        if sum == expected_sum {
            let sub = &numbers[0..set_size];
            return sub.iter().min().unwrap_or(&0) + sub.iter().max().unwrap_or(&0);
        }

        for i in 1..(numbers.len() - set_size) {
            sum -= numbers[i - 1];
            sum += numbers[i + set_size - 1];

            if sum == expected_sum {
                let sub = &numbers[i..(i + set_size - 1)];
                return sub.iter().min().unwrap_or(&0) + sub.iter().max().unwrap_or(&0);
            }
        }
    }

    0
}

pub fn run_part_2() {
    let numbers = read_numbers();
    let first_wrong_number = find_first_wrong_number(&numbers, 25);
    let result = find_encryption_weakness(&numbers, first_wrong_number.unwrap());

    assert_eq!(result, 13935797);
}
