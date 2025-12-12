use std::collections::{HashMap, HashSet, VecDeque};

use regex::Regex;

use crate::read_input;

fn parse_rule(rule: &str) -> (String, Vec<(String, usize)>) {
    let parts: Vec<&str> = rule.split(" bags contain ").collect();
    let container_color: String = parts[0].to_string();
    let mut contained_colors_counts: Vec<(String, usize)> = vec![];

    if parts[0] != "no other bags." {
        let re = Regex::new(r"(?P<n>[\d]+)\s(?P<color>[\w\s]+)\sbag").unwrap();
        let captures = re.captures_iter(parts[1]);
        for capture in captures {
            contained_colors_counts.push((
                String::from(&capture["color"]),
                capture["n"].parse::<usize>().unwrap(),
            ));
        }
    }

    (container_color, contained_colors_counts)
}

fn read_bags_contained_in() -> HashMap<String, Vec<String>> {
    let rules = read_input(2020, 7)
        .unwrap()
        .map_while(Result::ok)
        .collect::<Vec<String>>();

    let mut contained_in: HashMap<String, Vec<String>> = HashMap::new();

    for rule in rules {
        let (container_color, contained_colors) = parse_rule(&rule);

        for contained_color in contained_colors {
            if !contained_in.contains_key(&contained_color.0) {
                contained_in.insert(contained_color.0.clone(), vec![]);
            }

            let mut containers = contained_in.get(&contained_color.0).unwrap().clone();
            containers.push(container_color.clone());
            contained_in.insert(contained_color.0.clone(), containers);
        }
    }

    contained_in
}

fn read_containers() -> HashMap<String, Vec<(String, usize)>> {
    let rules = read_input(2020, 7)
        .unwrap()
        .map_while(Result::ok)
        .collect::<Vec<String>>();

    let mut containers: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    for rule in rules {
        let (container_color, contained_colors_counts) = parse_rule(&rule);
        containers.insert(container_color, contained_colors_counts);
    }

    containers
}

pub fn run_part_1() {
    let contained_in = read_bags_contained_in();
    let containers = contained_in.get("shiny gold").unwrap();

    let mut queue: VecDeque<String> = containers.clone().into_iter().collect();
    let mut count = queue.len();

    let mut color_processed: HashSet<String> = HashSet::new();
    for container in containers {
        color_processed.insert(container.to_string());
    }

    while !queue.is_empty() {
        let current_color = queue.pop_front().unwrap();
        if let Some(containers) = contained_in.get(&current_color) {
            for container in containers {
                if !color_processed.contains(&container.to_string()) {
                    count += 1;
                    queue.push_back(container.to_string());
                    color_processed.insert(container.to_string());
                }
            }
        }
    }

    assert_eq!(count, 278);
}

pub fn run_part_2() {
    let containers = read_containers();

    let mut count = 0;
    let mut current_queue: VecDeque<(String, usize)> = containers
        .get("shiny gold")
        .unwrap()
        .clone()
        .into_iter()
        .collect();
    let mut next_queue: VecDeque<(String, usize)> = VecDeque::new();

    while !current_queue.is_empty() {
        let (current_color, current_count) = current_queue.pop_front().unwrap();
        count += current_count;

        if let Some(next_colors_counts) = containers.get(&current_color) {
            for (next_color, next_count) in next_colors_counts {
                next_queue.push_back((next_color.to_string(), next_count * current_count));
            }
        }

        if current_queue.is_empty() {
            current_queue.append(&mut next_queue);
        }
    }

    assert_eq!(count, 45157);
}
