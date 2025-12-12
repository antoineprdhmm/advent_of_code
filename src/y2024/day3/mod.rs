use crate::read_input;
use lazy_static::lazy_static;
use regex::{Captures, Regex};

lazy_static! {
    static ref MUL_REGEX: Regex =
        Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Invalid Regex Pattern ");
    static ref DONT_DO_REGEX: Regex =
        Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").expect("Invalid Regex Pattern");
}

fn process_instruction(captures: Captures<'_>) -> i32 {
    let first_number = captures[1]
        .parse::<i32>()
        .expect("Failed to parse first number");
    let second_number = captures[2]
        .parse::<i32>()
        .expect("Failed to parse second number");

    first_number * second_number
}

pub fn run_part_1() {
    let lines = read_input(2024, 3).unwrap();

    let result = lines
        .map_while(Result::ok)
        .map(|line| {
            MUL_REGEX
                .captures_iter(&line)
                .map(|captures| process_instruction(captures))
                .sum::<i32>()
        })
        .sum::<i32>();

    println!("{result}");
}

pub fn run_part_2() {
    let lines = read_input(2024, 3).unwrap();
    let line = lines.map_while(Result::ok).collect::<String>();

    let result = DONT_DO_REGEX
        .captures_iter(&line)
        .fold((0, true), |(sum, enabled), captures| match &captures[0] {
            "do()" => (sum, true),
            "don't()" => (sum, false),
            _ => (
                if enabled {
                    sum + process_instruction(captures)
                } else {
                    sum
                },
                enabled,
            ),
        })
        .0;

    println!("{result}");
}
