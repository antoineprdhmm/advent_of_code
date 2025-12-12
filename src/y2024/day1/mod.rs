use crate::read_input;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self};

fn parse_input(lines: io::Lines<io::BufReader<File>>) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in lines.map_while(Result::ok) {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        left.push(parts[0].parse::<i32>().unwrap());
        right.push(parts[1].parse::<i32>().unwrap());
    }

    (left, right)
}

pub fn run_part_1() {
    let lines = read_input(2024, 1).unwrap();
    let (mut left, mut right) = parse_input(lines);

    left.sort();
    right.sort();

    let mut total_distance = 0;

    for (l, r) in left.iter().zip(right.iter()) {
        total_distance += (l - r).abs();
    }

    println!("{}", total_distance);
}

pub fn run_part_2() {
    let lines = read_input(2024, 1).unwrap();
    let (left, right) = parse_input(lines);

    let mut right_counts = HashMap::new();
    for r in right {
        right_counts.entry(r).and_modify(|e| *e += 1).or_insert(1);
    }

    let similarity_score = left.iter().fold(0, |acc, l| {
        let count = *right_counts.entry(*l).or_default();
        acc + (*l * count)
    });

    println!("{}", similarity_score);
}
