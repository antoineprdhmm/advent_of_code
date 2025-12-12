use std::collections::{HashMap, HashSet};

use crate::read_input;

fn read_entries() -> Vec<i32> {
    read_input(2020, 1)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

pub fn run_part_1() {
    let entries = read_entries();

    let set: HashSet<i32> = entries.iter().cloned().collect();

    for entry in entries {
        let diff = 2020 - entry;
        if set.contains(&diff) {
            let product = entry * diff;

            println!("{:} + {:} = 2020", entry, diff);
            println!("{}", product);
            assert_eq!(product, 877971);

            break;
        }
    }
}

pub fn run_part_2() {
    let entries = read_entries();

    let mut map = HashMap::new();

    for i in 0..entries.len() {
        for j in (i + 1)..entries.len() {
            map.insert(entries[i] + entries[j], [entries[i], entries[j]]);
        }
    }

    for entry in entries {
        let diff = 2020 - entry;
        if map.contains_key(&diff) {
            let values = map.get(&diff).unwrap();
            let product = entry * values[0] * values[1];

            println!("{:} + {:} + {:} = 2020", entry, values[0], values[1]);
            println!("{}", product);
            assert_eq!(product, 203481432);

            break;
        }
    }
}
