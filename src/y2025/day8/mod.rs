use std::collections::HashSet;

use crate::read_input;

type Coordinate = (usize, usize, usize);

fn read_boxes_coordinates() -> Vec<Coordinate> {
    read_input(2025, 8)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| {
            let v: Vec<usize> = line
                .split(',')
                .map(|v| v.parse::<usize>().unwrap())
                .collect();
            (v[0], v[1], v[2])
        })
        .collect()
}

#[derive(Debug)]
struct BoxPair {
    a: Coordinate,
    b: Coordinate,
    distance_squared: usize,
}

fn get_sorted_pairs(coordinates: &[Coordinate]) -> Vec<BoxPair> {
    let mut pairs = Vec::new();

    for i in 0..coordinates.len() {
        for j in (i + 1)..coordinates.len() {
            let (a, b) = (coordinates[i], coordinates[j]);
            let distance_squared = (a.0 as isize - b.0 as isize).pow(2) as usize
                + (a.1 as isize - b.1 as isize).pow(2) as usize
                + (a.2 as isize - b.2 as isize).pow(2) as usize;

            pairs.push(BoxPair {
                a,
                b,
                distance_squared,
            })
        }
    }

    pairs.sort_unstable_by_key(|p| p.distance_squared);
    pairs
}

fn handle_pair(
    pair: &BoxPair,
    connected_box: &mut HashSet<Coordinate>,
    circuits: &mut Vec<HashSet<Coordinate>>,
) {
    connected_box.insert(pair.a);
    connected_box.insert(pair.b);

    let a_idx = circuits.iter().position(|x| x.contains(&pair.a));
    let b_idx = circuits.iter().position(|x| x.contains(&pair.b));

    match (a_idx, b_idx) {
        (Some(a), Some(b)) if a == b => {
            // Already in same circuit
        }
        (Some(a), Some(b)) => {
            // Merge circuits
            let values: Vec<_> = circuits[b].drain().collect();
            circuits[a].extend(values);
            circuits.swap_remove(b);
        }
        (Some(a), None) => {
            circuits[a].insert(pair.b);
        }
        (None, Some(b)) => {
            circuits[b].insert(pair.a);
        }
        (None, None) => {
            circuits.push(HashSet::from([pair.a, pair.b]));
        }
    }
}

pub fn run_part_1() {
    let coordinates = read_boxes_coordinates();
    let pairs = get_sorted_pairs(&coordinates);

    let mut circuits: Vec<HashSet<Coordinate>> = Vec::new();
    let mut connected_box: HashSet<Coordinate> = HashSet::new();

    const NB_CONNECTIONS_TO_DO: usize = 1000;
    for i in 0..NB_CONNECTIONS_TO_DO {
        handle_pair(&pairs[i], &mut connected_box, &mut circuits);
    }

    let mut sizes = circuits
        .iter()
        .filter(|c| !c.is_empty())
        .map(|c| c.len())
        .collect::<Vec<usize>>();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    assert_eq!(105952, sizes[0] * sizes[1] * sizes[2]);
}

pub fn run_part_2() {
    let coordinates = read_boxes_coordinates();
    let nb_boxes = coordinates.len();
    let pairs: Vec<BoxPair> = get_sorted_pairs(&coordinates);

    let mut circuits: Vec<HashSet<Coordinate>> = Vec::new();
    let mut connected_box: HashSet<Coordinate> = HashSet::new();

    let mut i = 0;
    while connected_box.len() < nb_boxes || (circuits.len() > 1) {
        handle_pair(&pairs[i], &mut connected_box, &mut circuits);
        i += 1;
    }

    assert_eq!(975931446, pairs[i - 1].a.0 * pairs[i - 1].b.0)
}
