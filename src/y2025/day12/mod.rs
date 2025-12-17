use std::collections::HashMap;

use crate::read_input;

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    present_counts: HashMap<usize, usize>,
}

fn read_puzzle_input() -> Vec<Region> {
    let lines: Vec<String> = read_input(2025, 12)
        .unwrap()
        .map_while(Result::ok)
        .collect();

    lines[30..lines.len()]
        .iter()
        .map(|line| {
            let (size, counts_str) = line.split_once(": ").unwrap();
            let (width_str, height_str) = size.split_once("x").unwrap();
            let (width, height) = (
                width_str.parse::<usize>().unwrap(),
                height_str.parse::<usize>().unwrap(),
            );

            let present_counts = counts_str
                .split_whitespace()
                .map(|v| v.parse::<usize>().unwrap())
                .enumerate()
                .collect();

            Region {
                width,
                height,
                present_counts,
            }
        })
        .collect()
}

pub fn run_part_1() {
    // number of filled cells by shape
    let areas: HashMap<usize, usize> =
        HashMap::from([(0, 7), (1, 7), (2, 6), (3, 5), (4, 7), (5, 7)]);

    let regions = read_puzzle_input();

    println!("{:?}", regions.len()); // 1000 regions in total

    // Filter out regions that are impossible
    // -> total cells occupied by presents > total cells available
    let regions = regions
        .iter()
        .filter(|r| {
            let available = r.height * r.width;
            let total_needed: usize = r
                .present_counts
                .iter()
                .map(|(i, a)| areas.get(&i).unwrap() * a)
                .sum();

            total_needed <= available
        })
        .collect::<Vec<&Region>>();

    println!("{:?}", regions.len()); // 476 that could fit

    // Check how many of the 476 could work just by putting the presents side by side
    let regions = regions
        .iter()
        .filter(|r| {
            let available = r.height * r.width;
            let total_needed: usize = r.present_counts.iter().map(|(i, a)| 9 * a).sum();

            total_needed <= available
        })
        .collect::<Vec<&&Region>>();

    println!("{:?}", regions.len()); // 476 !!

    // So all the cases that are not impossible are possible
    // Answer = 476
}

pub fn run_part_2() {
    // Nothing to do in part 2
}
