use crate::read_input;

use std::ops::RangeInclusive;

fn read_ingredients() -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    let lines: Vec<String> = read_input(2025, 5).unwrap().map_while(Result::ok).collect();

    let mut ranges: Vec<RangeInclusive<_>> = Vec::new();
    let mut ids = Vec::new();

    let mut finished_processing_ranges = false;

    for line in lines {
        if line.is_empty() {
            finished_processing_ranges = true;
        } else {
            if finished_processing_ranges {
                ids.push(line.parse::<usize>().unwrap());
            } else {
                let (lo, hi) = line.split_once("-").unwrap();
                ranges.push(lo.parse::<usize>().unwrap()..=hi.parse::<usize>().unwrap());
            }
        }
    }

    (ranges, ids)
}

pub fn run_part_1() {
    let (ranges, ids) = read_ingredients();

    let fresh_count = ids
        .iter()
        .filter(|id| ranges.iter().any(|r| r.contains(&id)))
        .count();

    assert_eq!(fresh_count, 811);
}

pub fn run_part_2() {
    let (mut ranges, _) = read_ingredients();

    // sort the ranges by lower bounds
    ranges.sort_by_key(|r| *r.start());

    let merged =
        ranges
            .into_iter()
            .fold(Vec::new(), |mut acc: Vec<RangeInclusive<usize>>, range| {
                if let Some(last) = acc.last_mut() {
                    if last.contains(range.start()) {
                        *last = *last.start()..=*range.end().max(last.end());
                        return acc;
                    }
                }
                acc.push(range);
                acc
            });

    let count: usize = merged.iter().map(|r| r.end() - r.start() + 1).sum();

    assert_eq!(count, 338189277144473);
}
