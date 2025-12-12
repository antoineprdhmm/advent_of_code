use std::collections::HashMap;

mod dot;

use dot::generate_dot_file;

use crate::read_input;

fn parse_input() -> CrossedWiresSystem {
    let mut crossed_wires_system = CrossedWiresSystem::default();

    let lines = read_input(2024, 24).unwrap().map_while(Result::ok);

    let mut did_find_separator = false;
    lines.for_each(|line| {
        if line.is_empty() {
            did_find_separator = true;
        } else if did_find_separator {
            let (left_part, destination_key) = line.split_once(" -> ").unwrap();
            let left_parts: Vec<&str> = left_part.split(" ").collect();
            let gate = match left_parts[1] {
                "XOR" => Gate::Xor,
                "AND" => Gate::And,
                "OR" => Gate::Or,
                _ => panic!("Unexpected value for Gate"),
            };
            crossed_wires_system.operations.push(Operation {
                gate,
                destination_key: destination_key.to_string(),
                key1: left_parts[0].to_string(),
                key2: left_parts[2].to_string(),
            });
        } else {
            let (key, value) = line.split_once(": ").unwrap();
            crossed_wires_system
                .values
                .insert(key.to_string(), value.parse::<u8>().unwrap());
        }
    });

    crossed_wires_system
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Gate {
    Xor,
    Or,
    And,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Operation {
    gate: Gate,
    key1: String,
    key2: String,
    destination_key: String,
}

#[derive(Default)]
struct CrossedWiresSystem {
    values: HashMap<String, u8>,
    operations: Vec<Operation>,
}

impl CrossedWiresSystem {
    pub fn execute_operations(&mut self) {
        while !self.operations.is_empty() {
            let mut unprocessed_operations: Vec<Operation> = Vec::new();
            for operation in self.operations.iter().cloned() {
                if let Some(v1) = self.values.get(&operation.key1) {
                    if let Some(v2) = self.values.get(&operation.key2) {
                        let result = match operation.gate {
                            Gate::And => v1 & v2,
                            Gate::Xor => v1 ^ v2,
                            Gate::Or => v1 | v2,
                        };
                        self.values
                            .insert(operation.destination_key.clone(), result);
                        continue;
                    }
                }
                unprocessed_operations.push(operation);
            }
            self.operations = unprocessed_operations;
        }
    }

    pub fn get_output_number(&self) -> usize {
        let mut i = 0;
        let mut result: usize = 0;
        while let Some(v) = self.values.get(&format!("z{:02}", i)) {
            result ^= (*v as usize) << i;
            i += 1;
        }

        result
    }

    pub fn debug_expected_output(&self) {
        let mut i = 0;
        let mut x: usize = 0;
        while let Some(v) = self.values.get(&format!("x{:02}", i)) {
            x ^= (*v as usize) << i;
            i += 1;
        }

        let mut i = 0;
        let mut y: usize = 0;
        while let Some(v) = self.values.get(&format!("y{:02}", i)) {
            y ^= (*v as usize) << i;
            i += 1;
        }

        println!("{:b}", x + y);
    }
}

pub fn run_part_1() {
    let mut crossed_wires_system = parse_input();
    crossed_wires_system.execute_operations();
    let output = crossed_wires_system.get_output_number();
    println!("{:?}", output);
}

// to build the SVG: `dot -Tsvg  src/day24/day24.dot -o src/day24/day24.svg`
pub fn run_part_2() {
    let mut crossed_wires_system = parse_input();
    generate_dot_file(&crossed_wires_system);
    crossed_wires_system.debug_expected_output();
    crossed_wires_system.execute_operations();
    let output = crossed_wires_system.get_output_number();
    println!("{:b}", output);

    let mut swaps: Vec<&str> = vec!["mkk", "z10", "qbw", "z14", "wcb", "z34", "wjb", "cvp"];
    swaps.sort();
    println!("{:?}", swaps.join(","));
}
