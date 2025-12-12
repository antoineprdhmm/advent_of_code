use std::collections::HashMap;

use crate::read_input;

#[derive(Default)]
struct GroupCount {
    yes_count_by_answer: HashMap<char, usize>,
    group_size: usize,
}

fn read_yes_answers_count_of_groups() -> Vec<GroupCount> {
    let mut groups_yes_answers: Vec<GroupCount> = Vec::new();
    let mut current_group_yes_answers: GroupCount = GroupCount {
        yes_count_by_answer: HashMap::new(),
        group_size: 0,
    };

    let lines = read_input(2020, 6)
        .unwrap()
        .map_while(Result::ok)
        .collect::<Vec<String>>();

    for line in lines {
        if line.is_empty() {
            groups_yes_answers.push(current_group_yes_answers);
            current_group_yes_answers = GroupCount::default()
        } else {
            for c in line.chars() {
                current_group_yes_answers
                    .yes_count_by_answer
                    .entry(c)
                    .or_insert(0);

                let counter = current_group_yes_answers
                    .yes_count_by_answer
                    .insert(c, 0)
                    .unwrap()
                    + 1;
                current_group_yes_answers
                    .yes_count_by_answer
                    .insert(c, counter);
            }
            current_group_yes_answers.group_size += 1;
        }
    }

    groups_yes_answers.push(current_group_yes_answers);

    groups_yes_answers
}

pub fn run_part_1() {
    let groups_yes_answers = read_yes_answers_count_of_groups();

    let sum = groups_yes_answers
        .iter()
        .map(|group| group.yes_count_by_answer.len())
        .sum::<usize>();

    assert_eq!(sum, 6633);
}

pub fn run_part_2() {
    let groups_yes_answers = read_yes_answers_count_of_groups();

    let sum = groups_yes_answers
        .iter()
        .map(|group| {
            group
                .yes_count_by_answer
                .values()
                .filter(|v| **v == group.group_size)
                .count()
        })
        .sum::<usize>();

    assert_eq!(sum, 3202);
}
