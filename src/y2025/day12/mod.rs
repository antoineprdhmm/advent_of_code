use crate::read_input;

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    present_counts: Vec<usize>,
}

fn read_puzzle_input() -> Vec<Region> {
    let lines: Vec<String> = read_input(2025, 12)
        .unwrap()
        .map_while(Result::ok)
        .collect();

    lines[30..]
        .iter()
        .map(|line| {
            let (size, counts_str) = line.split_once(": ").unwrap();
            let (width_str, height_str) = size.split_once('x').unwrap();
            let (width, height) = (
                width_str.parse::<usize>().unwrap(),
                height_str.parse::<usize>().unwrap(),
            );

            let present_counts = counts_str
                .split_whitespace()
                .map(|v| v.parse::<usize>().unwrap())
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
    const AREAS: [usize; 6] = [7, 7, 6, 5, 7, 7];

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
                .enumerate()
                .map(|(i, a)| AREAS[i] * a)
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
            let total_needed: usize = r.present_counts.iter().map(|a| 9 * a).sum();

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
