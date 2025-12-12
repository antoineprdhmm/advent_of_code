use crate::read_input;
use std::collections::HashSet;

struct CalibrationEquation {
    test_value: usize,
    numbers: Vec<usize>,
}

fn parse_line(line: &str) -> CalibrationEquation {
    let (test_str, numbers_str) = line.split_once(": ").expect("Invalid line format");

    CalibrationEquation {
        test_value: test_str.parse().expect("Result is not usize"),
        numbers: numbers_str
            .split_whitespace()
            .map(|c| c.parse().expect("Component is not usize"))
            .collect(),
    }
}

fn concat_numbers(a: usize, b: usize) -> usize {
    let mut b_copy = b;
    let mut factor = 1;

    while b_copy > 0 {
        b_copy /= 10;
        factor *= 10;
    }

    a * factor + b
}

impl CalibrationEquation {
    fn is_valid(&self, with_concat: bool) -> bool {
        let mut intermediate_values: HashSet<usize> = HashSet::new();
        intermediate_values.insert(self.numbers[0]);

        for &num in self.numbers.iter().skip(1) {
            let current: Vec<_> = intermediate_values.drain().collect();

            for v in current {
                let a = v + num;
                if a <= self.test_value {
                    intermediate_values.insert(a);
                }

                let m = v * num;
                if m <= self.test_value {
                    intermediate_values.insert(m);
                }

                // operator for part 2
                if with_concat {
                    let c = concat_numbers(v, num);
                    if c <= self.test_value {
                        intermediate_values.insert(c);
                    }
                }
            }
        }

        intermediate_values.contains(&self.test_value)
    }
}

fn run_part(with_concat: bool) -> usize {
    let lines = read_input(2024, 7).unwrap();

    lines
        .map_while(Result::ok)
        .map(|line| parse_line(&line))
        .filter(|equation| equation.is_valid(with_concat))
        .map(|equation| equation.test_value)
        .sum()
}

pub fn run_part_1() {
    println!("{}", run_part(false));
}

pub fn run_part_2() {
    println!("{}", run_part(true));
}
