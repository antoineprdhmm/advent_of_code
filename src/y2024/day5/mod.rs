use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use crate::read_input;

fn are_pages_ordered(pages: &[usize], orderings: &HashMap<usize, Vec<usize>>) -> bool {
    let mut discovered: HashSet<usize> = HashSet::new();

    !pages.iter().any(|page| {
        if let Some(pages_must_appear_after) = orderings.get(page) {
            if pages_must_appear_after
                .iter()
                .any(|p| discovered.contains(p))
            {
                return true;
            }
        }
        discovered.insert(*page);
        false
    })
}

fn get_orderings() -> HashMap<usize, Vec<usize>> {
    let ordering_lines = read_input(2024, 5).unwrap();

    ordering_lines
        .map_while(Result::ok)
        .filter_map(|line| {
            let mut values = line.split('|');
            let key = values.next()?.parse().ok()?;
            let value = values.next()?.parse().ok()?;
            Some((key, value))
        })
        .fold(HashMap::new(), |mut map, (key, value)| {
            map.entry(key).or_default().push(value);
            map
        })
}

pub fn run_part_1() {
    let orderings = get_orderings();
    let updates_lines = read_input(2024, 5).unwrap();

    let result = updates_lines
        .map_while(Result::ok)
        .fold(0, |result, update_line| {
            let pages: Vec<usize> = update_line
                .split(',')
                .filter_map(|v| v.parse::<usize>().ok())
                .collect();

            if are_pages_ordered(&pages, &orderings) {
                let mid_value = pages[(pages.len() - 1) / 2];
                return result + mid_value;
            }

            result
        });

    println!("{result}");
}

fn reorder_pages(
    mut pages: Vec<usize>,
    orderings: &HashMap<&usize, HashSet<&usize>>,
) -> Vec<usize> {
    pages.sort_by(|a, b| {
        if let Some(o) = orderings.get(a) {
            if o.contains(b) {
                return Ordering::Less;
            }
        }
        Ordering::Greater
    });
    pages
}

pub fn run_part_2() {
    let orderings = get_orderings();
    let orderings_hashset = HashMap::from_iter(
        orderings
            .iter()
            .map(|(key, values)| (key, HashSet::from_iter(values))),
    );

    let updates_lines = read_input(2024, 5).unwrap();

    let result = updates_lines
        .map_while(Result::ok)
        .fold(0, |result, update_line| {
            let pages: Vec<usize> = update_line
                .split(',')
                .filter_map(|v| v.parse::<usize>().ok())
                .collect();

            if are_pages_ordered(&pages, &orderings) {
                return result;
            }

            let reordered_pages = reorder_pages(pages, &orderings_hashset);
            let mid_value = reordered_pages[(reordered_pages.len() - 1) / 2];

            result + mid_value
        });

    println!("{result}");
}
