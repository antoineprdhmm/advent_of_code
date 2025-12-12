use crate::read_input;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self};

fn read_stones() -> Vec<usize> {
    let lines: io::Lines<io::BufReader<File>> = read_input(2024, 11).unwrap();

    let line = lines
        .map_while(Result::ok)
        .last()
        .expect("Failed to read input line");

    line.split_whitespace()
        .map(|v| v.parse().expect("Should be integer"))
        .collect::<Vec<usize>>()
}

fn get_digit_count(mut n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    let mut count = 0;
    while n > 0 {
        count += 1;
        n /= 10;
    }
    count
}

fn handle_stone(stone: usize, blinks: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if blinks == 0 {
        return 1;
    }

    if let Some(known_result) = memo.get(&(stone, blinks)) {
        *known_result
    } else {
        let unknown_result = if stone == 0 {
            handle_stone(1, blinks - 1, memo)
        } else {
            let digits_count = get_digit_count(stone);
            if digits_count % 2 == 0 {
                let half = digits_count / 2;
                let divisor = 10_usize.pow(half as u32);
                let right = stone % divisor;
                let left = stone / divisor;

                return handle_stone(right, blinks - 1, memo)
                    + handle_stone(left, blinks - 1, memo);
            } else {
                handle_stone(stone * 2024, blinks - 1, memo)
            }
        };

        memo.entry((stone, blinks)).or_insert(unknown_result);
        unknown_result
    }
}

fn run(blinks: usize) {
    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();
    let stones = read_stones();

    let count = stones
        .into_iter()
        .map(|stone| handle_stone(stone, blinks, &mut memo))
        .sum::<usize>();

    println!("{}", count);
}

pub fn run_part_1() {
    run(25);
}

pub fn run_part_2() {
    run(75);
}
