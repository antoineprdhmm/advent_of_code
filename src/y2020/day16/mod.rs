use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

use crate::read_input;

#[derive(Debug)]
struct Rule {
    text: String,
    ranges: Vec<RangeInclusive<usize>>,
}

fn parse_input() -> (Vec<Rule>, Vec<usize>, Vec<Vec<usize>>) {
    let lines: Vec<String> = read_input(2020, 16)
        .unwrap()
        .map_while(Result::ok)
        .collect();

    let groups: Vec<Vec<String>> = lines
        .split(|line| line.is_empty())
        .map(|group| group.to_vec())
        .collect();

    let rules = groups[0]
        .iter()
        .map(|raw_rule| {
            let (text, raw_ranges) = raw_rule.split_once(": ").unwrap();
            let raw_ranges = raw_ranges.split(" or ");

            Rule {
                text: text.to_string(),
                ranges: raw_ranges
                    .map(|raw_range| {
                        let (start, end) = raw_range.split_once("-").unwrap();
                        start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap()
                    })
                    .collect(),
            }
        })
        .collect();

    let nearby_tickets = groups[2]
        .iter()
        .skip(1)
        .map(|row| {
            row.split(",")
                .map(|v| v.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let my_ticket = groups[1][1]
        .split(",")
        .map(|v| v.parse::<usize>().unwrap())
        .collect();

    (rules, my_ticket, nearby_tickets)
}

fn value_is_valid(rules: &[Rule], value: &usize) -> bool {
    rules
        .iter()
        .any(|rule| rule.ranges.iter().any(|range| range.contains(value)))
}

pub fn run_part_1() {
    let (rules, _, nearby_tickets) = parse_input();

    let invalid_values: Vec<usize> = nearby_tickets
        .iter()
        .flat_map(|nearby_ticket| {
            nearby_ticket
                .iter()
                .filter(|value| !value_is_valid(&rules, value))
                .copied()
                .collect::<Vec<usize>>()
        })
        .collect();

    let result = invalid_values.iter().sum::<usize>();
    assert_eq!(20975, result);
}

pub fn run_part_2() {
    let (rules, my_ticket, nearby_tickets) = parse_input();

    let valid_tickets = nearby_tickets
        .into_iter()
        .filter(|nearby_ticket| {
            !nearby_ticket
                .iter()
                .any(|value| !value_is_valid(&rules, value))
        })
        .collect::<Vec<Vec<usize>>>();

    let mut values_i_by_rule_i: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (rule_i, rule) in rules.iter().enumerate() {
        for value_i in 0..my_ticket.len() {
            if valid_tickets.iter().all(|ticket| {
                rule.ranges
                    .iter()
                    .any(|range| range.contains(&ticket[value_i]))
            }) {
                values_i_by_rule_i
                    .entry(rule_i)
                    .or_default()
                    .insert(value_i);
            }
        }
    }

    // Sort in order to have a vector where the first element is the rule matching only one
    // value, the second one the rule matching two value, etc ..
    // Keep track on the values already assigned, so there is only one available value for each rule.

    let mut rules_values = values_i_by_rule_i
        .iter()
        .map(|(&rule_i, values_i)| (rule_i, values_i.iter().copied().collect()))
        .collect::<Vec<(usize, Vec<usize>)>>();
    rules_values.sort_by_key(|v| v.1.len());

    let mut mapping: HashMap<usize, usize> = HashMap::new();
    let mut assigned: HashSet<usize> = HashSet::new();

    for (rule_i, values_i) in rules_values {
        let value_without_rule_assigned = values_i
            .iter()
            .find(|value_i| !assigned.contains(value_i))
            .unwrap();
        assigned.insert(*value_without_rule_assigned);
        mapping.insert(rule_i, *value_without_rule_assigned);
    }

    let mut result = 1;
    for (rule_i, value_i) in mapping {
        if rules[rule_i].text.starts_with("departure") {
            result *= my_ticket[value_i];
        }
    }

    assert_eq!(910339449193, result);
}

pub fn run_part_2_generic() {
    let (rules, my_ticket, nearby_tickets) = parse_input();

    let valid_tickets = nearby_tickets
        .into_iter()
        .filter(|nearby_ticket| {
            !nearby_ticket
                .iter()
                .any(|value| !value_is_valid(&rules, value))
        })
        .collect::<Vec<Vec<usize>>>();

    let mapping = search_rec(0, HashMap::new(), &valid_tickets, &rules);

    if let Some(mapping) = mapping {
        let mut result = 1;

        for (rule_i, value_i) in mapping {
            if rules[rule_i].text.starts_with("departure") {
                result *= my_ticket[value_i];
            }
        }

        println!("{:?}", result);
    }
}

// for value at index i (start at 0)
// try all possibilities -> all ranges that matches
// recursive call with i+1 and this range marked as "used"
// once i == len() -> stop, means found a possibility
fn search_rec(
    value_i: usize,
    rules_used_at: HashMap<usize, usize>,
    tickets: &[Vec<usize>],
    rules: &[Rule],
) -> Option<HashMap<usize, usize>> {
    if value_i >= tickets[0].len() {
        return Some(rules_used_at);
    }

    for (rule_i, rule) in rules.iter().enumerate() {
        if !rules_used_at.contains_key(&rule_i) {
            let rule_ok_for_all_ticket = tickets.iter().all(|ticket| {
                rule.ranges
                    .iter()
                    .any(|range| range.contains(&ticket[value_i]))
            });

            if rule_ok_for_all_ticket {
                let mut sub_rules_used_at = rules_used_at.clone();
                sub_rules_used_at.insert(rule_i, value_i);
                let mapping = search_rec(value_i + 1, sub_rules_used_at, tickets, rules);
                if let Some(mapping) = mapping {
                    if mapping.len() == rules.len() {
                        return Some(mapping);
                    }
                }
            }
        }
    }

    None
}
