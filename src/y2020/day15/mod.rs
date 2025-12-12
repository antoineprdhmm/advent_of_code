use std::collections::HashMap;

use crate::read_input;

fn read_starting_numbers() -> Vec<usize> {
    read_input(2020, 15)
        .unwrap()
        .map_while(Result::ok)
        .flat_map(|line| {
            line.split(",")
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect()
}

fn push_value(
    value: usize,
    turn_i: usize,
    values_count: &mut HashMap<usize, usize>,
    value_2_last_turns: &mut HashMap<usize, (usize, usize)>,
) {
    *values_count.entry(value).or_insert(0) += 1;
    let entry_2_last_turns = value_2_last_turns.entry(value).or_default();
    *entry_2_last_turns = (turn_i, entry_2_last_turns.0);
}

fn run(nb_turns: usize) -> usize {
    let mut last_turn: usize = 0;
    let mut values_count: HashMap<usize, usize> = HashMap::new();
    let mut value_2_last_turns: HashMap<usize, (usize, usize)> = HashMap::new();

    let starting_numbers = read_starting_numbers();
    for (turn_i, value) in starting_numbers.iter().enumerate() {
        last_turn = *value;
        push_value(
            *value,
            turn_i + 1,
            &mut values_count,
            &mut value_2_last_turns,
        );
    }

    for turn_i in starting_numbers.len()..nb_turns {
        if values_count.get(&last_turn) == Some(&1) {
            last_turn = 0;
            push_value(0, turn_i + 1, &mut values_count, &mut value_2_last_turns);
        } else {
            let entry_2_last_turns = value_2_last_turns.entry(last_turn).or_default();
            let v = entry_2_last_turns.0 - entry_2_last_turns.1;
            last_turn = v;
            push_value(v, turn_i + 1, &mut values_count, &mut value_2_last_turns);
        }
    }

    last_turn
}

pub fn run_part_1() {
    assert_eq!(319, run(2020));
}

pub fn run_part_2() {
    assert_eq!(2424, run(30000000));
}
