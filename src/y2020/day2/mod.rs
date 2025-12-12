use crate::read_input;

#[derive(Debug)]
struct PasswordInput {
    password: String,
    letter: char,
    first_value: usize,
    second_value: usize,
}

fn read_password_inputs() -> Vec<PasswordInput> {
    read_input(2020, 2)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| {
            let line_parts: Vec<&str> = line.split_whitespace().collect();

            let mut parts = line_parts[0].split("-");
            let min_occ_string = parts.next().unwrap().to_string();
            let max_occ_string = parts.next().unwrap().to_string();

            let password: String = line_parts[2].into();

            let letter: char = line_parts[1]
                .split(":")
                .next()
                .unwrap()
                .chars()
                .next()
                .unwrap();

            PasswordInput {
                first_value: min_occ_string.parse::<usize>().unwrap(),
                second_value: max_occ_string.parse::<usize>().unwrap(),
                password,
                letter,
            }
        })
        .collect()
}

fn count_valid_passwords_part_1(password_inputs: &[PasswordInput]) -> usize {
    password_inputs
        .iter()
        .filter(|input| {
            let count = input
                .password
                .chars()
                .filter(|c| *c == input.letter)
                .count();
            count <= input.second_value && count >= input.first_value
        })
        .count()
}

pub fn run_part_1() {
    let password_inputs = read_password_inputs();
    let count = count_valid_passwords_part_1(&password_inputs);
    println!("Count = {}", count);
    assert_eq!(count, 538);
}

fn count_valid_passwords_part_2(password_inputs: &[PasswordInput]) -> usize {
    password_inputs
        .iter()
        .filter(|input| {
            let chars: Vec<char> = input.password.chars().collect();
            (chars[input.first_value - 1] == input.letter
                || chars[input.second_value - 1] == input.letter)
                && chars[input.first_value - 1] != chars[input.second_value - 1]
        })
        .count()
}

pub fn run_part_2() {
    let password_inputs = read_password_inputs();
    let count = count_valid_passwords_part_2(&password_inputs);
    println!("Count = {}", count);
    assert_eq!(count, 489);
}
