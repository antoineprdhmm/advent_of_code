use crate::read_input;
use std::ops::Div;

#[derive(Clone)]
struct Instruction {
    opcode: u8,
    operand: u8,
}

struct Computer {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    program: Vec<Instruction>,
}

impl Computer {
    fn get_combo_operand_value(&self, operand: u8) -> usize {
        match operand {
            0..=3 => operand.into(),
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("Unsupported combo operand"),
        }
    }

    pub fn run_program(&mut self) -> Vec<u8> {
        let mut pointer = 0;
        let mut output = Vec::new();

        while pointer < self.program.len() {
            let instruction = &self.program[pointer];
            let mut has_jumped = false;
            match instruction.opcode {
                0 => {
                    self.register_a = self.register_a.div(
                        2_usize.pow(
                            self.get_combo_operand_value(instruction.operand)
                                .try_into()
                                .unwrap(),
                        ),
                    );
                }
                1 => {
                    self.register_b ^= usize::from(instruction.operand);
                }
                2 => {
                    self.register_b = self.get_combo_operand_value(instruction.operand) % 8;
                }
                3 => {
                    if self.register_a != 0 {
                        has_jumped = true;
                        pointer = instruction.operand as usize / 2;
                    }
                }
                4 => {
                    self.register_b ^= self.register_c;
                }
                5 => {
                    output.push((self.get_combo_operand_value(instruction.operand) % 8) as u8);
                }
                6 => {
                    self.register_b = self.register_a.div(
                        2_usize.pow(
                            self.get_combo_operand_value(instruction.operand)
                                .try_into()
                                .unwrap(),
                        ),
                    );
                }
                7 => {
                    self.register_c = self.register_a.div(
                        2_usize.pow(
                            self.get_combo_operand_value(instruction.operand)
                                .try_into()
                                .unwrap(),
                        ),
                    );
                }
                _ => panic!("opcode not supported"),
            }

            if !has_jumped {
                pointer += 1;
            }
        }

        output
    }
}

fn parse_register(s: &str) -> usize {
    s.split_once(": ").unwrap().1.parse::<usize>().unwrap()
}

fn init_computer() -> Computer {
    let input = read_input(2024, 17)
        .unwrap()
        .map_while(Result::ok)
        .collect::<Vec<String>>();

    Computer {
        register_a: parse_register(&input[0]),
        register_b: parse_register(&input[1]),
        register_c: parse_register(&input[2]),
        program: input[4]
            .split_once(": ")
            .unwrap()
            .1
            .split(",")
            .map(|v| v.parse::<u8>().unwrap())
            .collect::<Vec<u8>>()
            .chunks(2)
            .map(|chunk| Instruction {
                opcode: chunk[0],
                operand: chunk[1],
            })
            .collect(),
    }
}

pub fn run_part_1() {
    let mut computer = init_computer();
    let output = computer.run_program();
    println!(
        "{}",
        output
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(",")
    )
}

fn run_instructions(a: usize) -> usize {
    let b = (a % 8) ^ 1;
    let c = a.div(2_usize.pow(b as u32));
    ((b ^ c) ^ 6) % 8
}

pub fn run_part_2() {
    let mut targets = [2, 4, 1, 1, 7, 5, 4, 0, 0, 3, 1, 6, 5, 5, 3, 0];
    targets.reverse();

    let mut a_candidates = [0].to_vec();

    for target in targets {
        let mut next_a_candidates = Vec::new();
        for xxx in 0..8 {
            for candidate in a_candidates.iter() {
                let a = candidate << 3 | xxx;
                let output = run_instructions(a);
                if output == target {
                    next_a_candidates.push(a);
                }
            }
        }

        a_candidates = next_a_candidates;
    }

    println!("{}", a_candidates.iter().min().unwrap());
}
