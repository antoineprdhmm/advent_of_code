use std::collections::HashMap;

use crate::read_input;

struct Mask {
    pub and_mask: usize,
    pub or_mask: usize,
    raw_mask: String,
}

impl Mask {
    fn new(raw_mask: &str) -> Self {
        Mask {
            and_mask: Self::create_and_mask(raw_mask),
            or_mask: Self::create_or_mask(raw_mask),
            raw_mask: raw_mask.to_string(),
        }
    }

    /**
     * Create a mask to set 0 where the mask is 0
     * & 1 -> keep the same value unchanged
     * & 0 -> set 0
     */
    pub fn create_and_mask(raw_mask: &str) -> usize {
        let mut and_mask: usize = 0;
        for c in raw_mask.chars() {
            and_mask <<= 1;
            if c == 'X' || c == '1' {
                and_mask |= 1;
            }
        }
        and_mask
    }

    /**
     * Create a mask to set 1 where the mask is 1
     * | 1 -> set 1
     * | 0 -> keep the same value unchanged
     */
    pub fn create_or_mask(raw_mask: &str) -> usize {
        let mut or_mask: usize = 0;
        for c in raw_mask.chars() {
            or_mask <<= 1;
            if c == '1' {
                or_mask |= 1;
            }
        }
        or_mask
    }

    pub fn apply_floating_bits(&self, address: usize) -> Vec<usize> {
        let mut queue = vec![address];

        let chars: Vec<char> = self.raw_mask.chars().collect();
        for i in 0..chars.len() {
            // everytime the char is X in the mask
            // there are 2 possibilities for each address:
            // - one where the X is a 0
            // - another where the X is a 1
            if chars[i] == 'X' {
                let mut next_queue = Vec::new();
                for item in queue {
                    next_queue.push(item | (1 << (chars.len() - 1 - i)));
                    next_queue.push(item & !(1 << (chars.len() - 1 - i)));
                }
                queue = next_queue;
            }
        }

        queue
    }
}

struct MemoryInstruction {
    address: usize,
    value: usize,
}

enum Instruction {
    Mask(Mask),
    Memory(MemoryInstruction),
}

fn read_program() -> Vec<Instruction> {
    read_input(2020, 14)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| {
            let parts = line.split_once(" = ").unwrap();
            if parts.0 == "mask" {
                Instruction::Mask(Mask::new(parts.1))
            } else {
                let mut address = parts.0.split_once("[").unwrap().1.to_string();
                address.pop();
                Instruction::Memory(MemoryInstruction {
                    address: address.parse().unwrap(),
                    value: parts.1.parse().unwrap(),
                })
            }
        })
        .collect()
}

pub fn run_part_1() {
    let program = read_program();

    let mut current_mask = None;
    let mut memory = HashMap::new();

    for instruction in program {
        match instruction {
            Instruction::Mask(mask) => {
                current_mask = Some(mask);
            }
            Instruction::Memory(instruction) => {
                if let Some(mask) = &current_mask {
                    memory.insert(
                        instruction.address,
                        (instruction.value & mask.and_mask) | mask.or_mask,
                    );
                }
            }
        }
    }

    let result: usize = memory.values().sum();
    assert_eq!(result, 14722016054794);
}

pub fn run_part_2() {
    let program = read_program();

    let mut current_mask = None;
    let mut memory = HashMap::new();

    for instruction in program {
        match instruction {
            Instruction::Mask(mask) => {
                current_mask = Some(mask);
            }
            Instruction::Memory(instruction) => {
                if let Some(mask) = &current_mask {
                    for address in mask.apply_floating_bits(instruction.address | mask.or_mask) {
                        memory.insert(address, instruction.value);
                    }
                }
            }
        }
    }

    let result: usize = memory.values().sum();
    assert_eq!(result, 3618217244644);
}
