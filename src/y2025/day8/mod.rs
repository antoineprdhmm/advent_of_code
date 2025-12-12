use std::collections::HashSet;

use crate::read_input;

fn read_boxes_coordinates() -> Vec<Coordinate> {
    read_input(2025, 8)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| {
            let v: Vec<usize> = line
                .split(",")
                .map(|v| v.parse::<usize>().unwrap())
                .collect();
            (v[0], v[1], v[2])
        })
        .collect()
}

type Coordinate = (usize, usize, usize);

#[derive(Debug)]
struct BoxPair {
    a: Coordinate,
    b: Coordinate,
    distance: f64,
}

fn get_sorted_pairs(coordinates: Vec<(usize, usize, usize)>) -> Vec<BoxPair> {
    let mut pairs = Vec::new();

    for i in 0..coordinates.len() {
        for j in (i + 1)..coordinates.len() {
            let distance = (((coordinates[i].0 as isize - coordinates[j].0 as isize).pow(2)
                + (coordinates[i].1 as isize - coordinates[j].1 as isize).pow(2)
                + (coordinates[i].2 as isize - coordinates[j].2 as isize).pow(2))
                as f64)
                .sqrt();

            pairs.push(BoxPair {
                a: coordinates[i],
                b: coordinates[j],
                distance,
            })
        }
    }

    pairs.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    pairs
}

fn handle_pair(
    pair: &BoxPair,
    connected_box: &mut HashSet<Coordinate>,
    circuits: &mut Vec<HashSet<Coordinate>>,
) {
    connected_box.insert(pair.a);
    connected_box.insert(pair.b);

    let a_circuit = &circuits.iter().position(|x| x.contains(&pair.a));
    let b_circuit = &circuits.iter().position(|x| x.contains(&pair.b));

    if let Some(a) = a_circuit {
        if let Some(b) = b_circuit {
            // Both are in a set already
            if a == b {
                // Already connected together, nothing to do
                return;
            } else {
                // Not connected -> let's merge the sets
                let values = circuits[*b].clone();
                circuits[*a].extend(values);
                circuits.swap_remove(*b);
                return;
            }
        }
    }

    if a_circuit.is_none() && b_circuit.is_none() {
        // none are in a circuit, let's create a new circuit
        circuits.push(HashSet::from_iter(vec![pair.a, pair.b]));
        return;
    }

    // only one set, add the other to the circuit
    if a_circuit.is_none() {
        if let Some(b) = b_circuit {
            circuits[*b].insert(pair.a);
            return;
        }
    } else {
        if let Some(a) = a_circuit {
            circuits[*a].insert(pair.b);
            return;
        }
    }
}

pub fn run_part_1() {
    let coordinates = read_boxes_coordinates();
    let pairs = get_sorted_pairs(coordinates);

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
    sizes.sort();
    sizes.reverse();

    assert_eq!(105952, sizes[0] * sizes[1] * sizes[2]);
}

pub fn run_part_2() {
    let coordinates = read_boxes_coordinates();
    let nb_boxes = coordinates.len();
    let pairs: Vec<BoxPair> = get_sorted_pairs(coordinates);

    let mut circuits: Vec<HashSet<Coordinate>> = Vec::new();
    let mut connected_box: HashSet<Coordinate> = HashSet::new();

    let mut i = 0;
    while connected_box.len() < nb_boxes || (circuits.len() > 1) {
        handle_pair(&pairs[i], &mut connected_box, &mut circuits);
        i += 1;
    }

    assert_eq!(975931446, pairs[i - 1].a.0 * pairs[i - 1].b.0)
}
