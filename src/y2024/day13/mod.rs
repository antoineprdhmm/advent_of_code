use crate::read_input;
use std::fs::File;
use std::io::{self};

#[derive(Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

struct ClawConfiguration {
    a: Coordinate,
    b: Coordinate,
    prize: Coordinate,
}

impl ClawConfiguration {
    fn compute_determinant(v1: &Coordinate, v2: &Coordinate) -> f64 {
        v1.x as f64 * v2.y as f64 - v1.y as f64 * v2.x as f64
    }

    pub fn adjust_prize(&mut self, prize_adjustment: usize) {
        self.prize.x += prize_adjustment;
        self.prize.y += prize_adjustment;
    }

    // Using Cramer's Rule
    pub fn count_tokens(&self) -> f64 {
        let d = Self::compute_determinant(&self.a, &self.b);
        let d_a = Self::compute_determinant(&self.prize, &self.b);
        let d_b = Self::compute_determinant(&self.a, &self.prize);

        (d_a / d) * 3.0 + (d_b / d)
    }
}

fn parse_coordinate(s: &str, delimiter: &str) -> usize {
    s.split_once(delimiter).unwrap().1.parse::<usize>().unwrap()
}

fn extract_button_coordinates(s: &str, delimiter: &str) -> Coordinate {
    let (x, y) = s.split_once(": ").unwrap().1.split_once(", ").unwrap();

    Coordinate {
        x: parse_coordinate(x, delimiter),
        y: parse_coordinate(y, delimiter),
    }
}

fn read_configurations() -> Vec<ClawConfiguration> {
    let lines: io::Lines<io::BufReader<File>> = read_input(2024, 13).unwrap();

    let all_lines = lines
        .map_while(Result::ok)
        .filter(|line| !line.is_empty())
        .collect::<Vec<String>>();

    all_lines
        .chunks(3)
        .map(|chunk| ClawConfiguration {
            a: extract_button_coordinates(&chunk[0], "+"),
            b: extract_button_coordinates(&chunk[1], "+"),
            prize: extract_button_coordinates(&chunk[2], "="),
        })
        .collect::<Vec<_>>()
}

pub fn run(prize_adjustment: usize) -> usize {
    read_configurations()
        .into_iter()
        .map(|mut config| {
            config.adjust_prize(prize_adjustment);
            config
        })
        .map(|config| config.count_tokens())
        .filter(|count| count.floor() == *count)
        .map(|count| count as usize)
        .sum::<usize>()
}

pub fn run_part_1() {
    let count = run(0);
    println!("{count}");
}

pub fn run_part_2() {
    let count = run(10000000000000);
    println!("{count}");
}
