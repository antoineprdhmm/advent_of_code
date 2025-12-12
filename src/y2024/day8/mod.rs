use std::collections::{HashMap, HashSet};

use crate::read_input;

type Point = (isize, isize);

struct Input {
    antennas_by_label: HashMap<char, Vec<Point>>,
    nb_rows: isize,
    nb_cols: isize,
}

fn read_antennas_input() -> Input {
    let lines = read_input(2024, 8).unwrap();
    let lines: Vec<_> = lines.map_while(Result::ok).collect();

    let mut antennas_by_label: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch != '.' {
                antennas_by_label
                    .entry(ch)
                    .or_default()
                    .push((row as isize, col as isize));
            }
        }
    }

    Input {
        antennas_by_label,
        nb_rows: lines.len().try_into().unwrap(),
        nb_cols: lines[0].len().try_into().unwrap(),
    }
}

fn is_antinode_in_bounds(nb_rows: &isize, nb_cols: &isize, antinode: &Point) -> bool {
    antinode.0 >= 0 && antinode.1 >= 0 && antinode.0 < *nb_rows && antinode.1 < *nb_cols
}

pub fn run_part_1() {
    let Input {
        antennas_by_label,
        nb_rows,
        nb_cols,
    } = read_antennas_input();

    let mut antinodes: HashSet<Point> = HashSet::new();

    for positions in antennas_by_label.values() {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let p1 = positions[i];
                let p2 = positions[j];
                let diff_row = p1.0 - p2.0;
                let diff_col = p1.1 - p2.1;
                let p1_antinode = (p1.0 + diff_row, p1.1 + diff_col);
                let p2_antinode = (p2.0 - diff_row, p2.1 - diff_col);

                if is_antinode_in_bounds(&nb_rows, &nb_cols, &p1_antinode) {
                    antinodes.insert(p1_antinode);
                }
                if is_antinode_in_bounds(&nb_rows, &nb_cols, &p2_antinode) {
                    antinodes.insert(p2_antinode);
                }
            }
        }
    }

    println!("{}", antinodes.len());
}

fn generate_antinodes(
    nb_rows: &isize,
    nb_cols: &isize,
    p: (isize, isize),
    shift: (isize, isize),
) -> Vec<(isize, isize)> {
    let mut antinode = (p.0 + shift.0, p.1 + shift.1);
    let mut antinodes: Vec<(isize, isize)> = Vec::new();

    while is_antinode_in_bounds(nb_rows, nb_cols, &antinode) {
        antinodes.push(antinode);
        antinode = (antinode.0 + shift.0, antinode.1 + shift.1);
    }

    antinodes
}

pub fn run_part_2() {
    let Input {
        antennas_by_label,
        nb_rows,
        nb_cols,
    } = read_antennas_input();

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    for positions in antennas_by_label.values() {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let p1 = positions[i];
                let p2 = positions[j];
                let diff_row = p1.0 - p2.0;
                let diff_col = p1.1 - p2.1;

                antinodes.extend(generate_antinodes(
                    &nb_rows,
                    &nb_cols,
                    p1,
                    (-diff_row, -diff_col),
                ));
                antinodes.extend(generate_antinodes(
                    &nb_rows,
                    &nb_cols,
                    p2,
                    (diff_row, diff_col),
                ));
            }
        }
    }

    println!("{}", antinodes.len());
}
