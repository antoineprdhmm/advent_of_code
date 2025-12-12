use std::collections::HashSet;

use crate::read_input;

#[derive(Clone)]
struct Instruction {
    operation: String,
    argument: isize,
}

fn read_instructions() -> Vec<Instruction> {
    read_input(2020, 8)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| {
            let line_parts: Vec<&str> = line.split(" ").collect();

            let operation: String = line_parts[0].to_string();
            let argument = line_parts[1].parse::<isize>().unwrap();

            Instruction {
                operation,
                argument,
            }
        })
        .collect()
}

fn find_accumulator_value(instructions: &[Instruction]) -> isize {
    let mut accumulator: isize = 0;
    let mut current_instruction_index: usize = 0;
    let mut already_visited_instructions: HashSet<usize> = HashSet::new();

    while !already_visited_instructions.contains(&current_instruction_index) {
        already_visited_instructions.insert(current_instruction_index);

        let Instruction {
            operation,
            argument,
        } = &instructions[current_instruction_index];

        if operation == "nop" {
            current_instruction_index = (current_instruction_index + 1) % instructions.len();
        } else if operation == "jmp" {
            let next_index_without_mod = current_instruction_index as isize + argument;

            if next_index_without_mod >= 0 {
                current_instruction_index = (next_index_without_mod as usize) % instructions.len();
            } else {
                current_instruction_index = ((next_index_without_mod % instructions.len() as isize)
                    + instructions.len() as isize)
                    as usize;
            }
        } else {
            accumulator += argument;
            current_instruction_index = (current_instruction_index + 1) % instructions.len();
        }
    }

    accumulator
}

fn is_infinite_loop(instructions: &[Instruction]) -> (bool, isize) {
    let mut accumulator: isize = 0;
    let mut current_instruction_index: usize = 0;
    let mut already_visited_instructions: HashSet<usize> = HashSet::new();

    while !already_visited_instructions.contains(&current_instruction_index) {
        already_visited_instructions.insert(current_instruction_index);

        let Instruction {
            operation,
            argument,
        } = &instructions[current_instruction_index];

        if operation == "acc" {
            accumulator += argument;
        }

        if operation == "nop" || operation == "acc" {
            current_instruction_index += 1;
            if current_instruction_index == instructions.len() {
                return (false, accumulator);
            }
            current_instruction_index %= instructions.len();
        } else {
            let next_index_without_mod = current_instruction_index as isize + argument;

            if next_index_without_mod >= 0 {
                if next_index_without_mod >= instructions.len() as isize {
                    current_instruction_index =
                        (next_index_without_mod as usize) % instructions.len();
                    if current_instruction_index == 0 {
                        return (false, accumulator);
                    }
                } else {
                    current_instruction_index =
                        (next_index_without_mod as usize) % instructions.len();
                }
            } else {
                current_instruction_index = ((next_index_without_mod % instructions.len() as isize)
                    + instructions.len() as isize)
                    as usize;
            }
        }
    }

    (true, accumulator)
}

fn fix_instructions(instructions: &mut [Instruction]) -> isize {
    // try all possible fixes and see if it fixes the infinite loop
    for i in 0..instructions.len() {
        if instructions[i].operation == "nop" || instructions[i].operation == "jmp" {
            let memo = if instructions[i].operation == "nop" {
                instructions[i].operation = String::from("jmp");
                "nop"
            } else {
                instructions[i].operation = String::from("nop");
                "jmp"
            };

            let (is_infinite, acc) = is_infinite_loop(instructions);
            if !is_infinite {
                return acc;
            }

            instructions[i].operation = memo.to_string();
        }
    }

    0
}

pub fn run_part_1() {
    let instructions = read_instructions();
    let accumulator = find_accumulator_value(&instructions);

    assert_eq!(accumulator, 2058);
}

pub fn run_part_2() {
    let mut instructions = read_instructions();
    let accumulator = fix_instructions(&mut instructions);

    assert_eq!(accumulator, 1000);
}
