use core::panic;

use crate::read_input;

fn read_beam_splitting() -> Vec<Vec<char>> {
    read_input(2025, 7)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| line.chars().collect())
        .collect()
}

pub fn run_part_1() {
    let mut beam_splitting = read_beam_splitting();

    // Init line 2 with | under S
    let s_idx = beam_splitting[0].iter().position(|&x| x == 'S').unwrap();
    beam_splitting[1][s_idx] = '|';

    let mut split_count = 0;

    // Run the splitting logic on the next lines
    for i in 2..beam_splitting.len() {
        for j in 0..beam_splitting[i].len() {
            match beam_splitting[i][j] {
                '.' => {
                    if beam_splitting[i - 1][j] == '|' {
                        beam_splitting[i][j] = '|';
                    }
                }
                '^' => {
                    if beam_splitting[i - 1][j] == '|' {
                        beam_splitting[i][j - 1] = '|';
                        beam_splitting[i][j + 1] = '|';
                        split_count += 1;
                    }
                }
                '|' => (),
                c => panic!("Unexpected char '{:?}'", c),
            }
        }
    }

    assert_eq!(split_count, 1640);
}

pub fn run_part_2() {
    let mut beam_splitting = read_beam_splitting();

    // Init line 2 with | under S
    let s_idx = beam_splitting[0].iter().position(|&x| x == 'S').unwrap();
    beam_splitting[1][s_idx] = '|';

    // Keep track of the number of possible paths for each beam on each line
    let mut paths_count: Vec<Vec<usize>> = Vec::new();

    // useless, just more convenient to use the same indexes for lines as beam_splitting later
    paths_count.push(vec![0; beam_splitting[1].len()]);

    let mut default_counts = vec![0; beam_splitting[1].len()];
    default_counts[s_idx] = 1;
    paths_count.push(default_counts);

    // Run the splitting logic on the next lines
    for i in 2..beam_splitting.len() {
        // Init counts for the current line
        paths_count.push(vec![0; beam_splitting[0].len()]);

        for j in 0..beam_splitting[i].len() {
            match beam_splitting[i][j] {
                '.' => {
                    if beam_splitting[i - 1][j] == '|' {
                        beam_splitting[i][j] = '|';
                        paths_count[i][j] += paths_count[i - 1][j]
                    }
                }
                '^' => {
                    if beam_splitting[i - 1][j] == '|' {
                        beam_splitting[i][j - 1] = '|';
                        beam_splitting[i][j + 1] = '|';

                        paths_count[i][j - 1] += paths_count[i - 1][j];
                        paths_count[i][j + 1] += paths_count[i - 1][j];
                    }
                }
                '|' => {
                    // Special case: beams merge
                    // . | |
                    // | ^ |
                    if beam_splitting[i - 1][j] == '|' {
                        paths_count[i][j] += paths_count[i - 1][j];
                    }
                }
                c => panic!("Unexpected char '{:?}'", c),
            }
        }
    }

    let total: usize = paths_count.iter().last().unwrap().iter().sum();
    assert_eq!(total, 40999072541589);
}
