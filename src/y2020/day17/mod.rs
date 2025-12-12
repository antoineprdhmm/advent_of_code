use std::collections::{HashMap, HashSet};

use crate::read_input;

fn read_initial_3d_state() -> HashSet<(isize, isize, isize)> {
    read_input(2020, 17)
        .unwrap()
        .map_while(Result::ok)
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == '#' {
                        Some((x as isize, y as isize, 0))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn read_initial_4d_state() -> HashSet<(isize, isize, isize, isize)> {
    read_initial_3d_state()
        .iter()
        .map(|(x, y, z)| (*x, *y, *z, 1))
        .collect()
}

const NB_CYCLES: u32 = 6;

pub fn run_part_1() {
    let mut state = read_initial_3d_state();

    for _ in 0..NB_CYCLES {
        let mut cubes_active_neighbors: HashMap<(isize, isize, isize), usize> = HashMap::new();

        for active_cube in &state {
            for x in [-1, 0, 1] {
                for y in [-1, 0, 1] {
                    for z in [-1, 0, 1] {
                        if !(x == 0 && y == 0 && z == 0) {
                            *cubes_active_neighbors
                                .entry((active_cube.0 + x, active_cube.1 + y, active_cube.2 + z))
                                .or_default() += 1;
                        }
                    }
                }
            }
        }

        state = cubes_active_neighbors
            .iter()
            .filter_map(|(k, v)| {
                if *v == 3 || (state.contains(k) && *v == 2) {
                    Some(*k)
                } else {
                    None
                }
            })
            .collect::<HashSet<(isize, isize, isize)>>();
    }

    assert_eq!(state.len(), 319);
}

pub fn run_part_2() {
    let mut state = read_initial_4d_state();

    for _ in 0..NB_CYCLES {
        let mut cubes_active_neighbors: HashMap<(isize, isize, isize, isize), usize> =
            HashMap::new();

        for active_cube in &state {
            for x in [-1, 0, 1] {
                for y in [-1, 0, 1] {
                    for z in [-1, 0, 1] {
                        for w in [-1, 0, 1] {
                            if !(x == 0 && y == 0 && z == 0 && w == 0) {
                                *cubes_active_neighbors
                                    .entry((
                                        active_cube.0 + x,
                                        active_cube.1 + y,
                                        active_cube.2 + z,
                                        active_cube.3 + w,
                                    ))
                                    .or_default() += 1;
                            }
                        }
                    }
                }
            }
        }

        state = cubes_active_neighbors
            .iter()
            .filter_map(|(k, v)| {
                if *v == 3 || (state.contains(k) && *v == 2) {
                    Some(*k)
                } else {
                    None
                }
            })
            .collect::<HashSet<(isize, isize, isize, isize)>>();
    }

    assert_eq!(state.len(), 2324);
}
