use crate::read_input;

#[derive(Clone, Debug)]
struct Range {
    pub lower_bound: usize,
    pub upper_bound: usize,
}

fn read_ingredients() -> (Vec<Range>, Vec<usize>) {
    let lines: Vec<String> = read_input(2025, 5).unwrap().map_while(Result::ok).collect();

    let mut ranges = Vec::new();
    let mut ids = Vec::new();

    let mut finished_processing_ranges = false;

    for line in lines {
        if line == "" {
            finished_processing_ranges = true;
        } else {
            if finished_processing_ranges {
                ids.push(line.parse::<usize>().unwrap());
            } else {
                let (lower_bound_str, upper_bound_str) = line.split_once("-").unwrap();
                ranges.push(Range {
                    lower_bound: lower_bound_str.parse::<usize>().unwrap(),
                    upper_bound: upper_bound_str.parse::<usize>().unwrap(),
                });
            }
        }
    }

    (ranges, ids)
}

pub fn run_part_1() {
    let (ranges, ids) = read_ingredients();

    let mut fresh_count = 0;

    for id in ids {
        for range in &ranges {
            if id == range.lower_bound
                || id == range.upper_bound
                || (id > range.lower_bound && id < range.upper_bound)
            {
                fresh_count += 1;
                break;
            }
        }
    }

    assert_eq!(fresh_count, 811);
}

pub fn run_part_2() {
    let (mut ranges, _) = read_ingredients();

    // sort the ranges by lower bounds
    ranges.sort_by(|a, b| a.lower_bound.cmp(&b.lower_bound));

    let mut did_change = true;
    while did_change {
        let mut next_ranges = Vec::new();
        let mut skip_next = false;
        did_change = false;

        for i in 0..(ranges.len() - 1) {
            if skip_next {
                skip_next = false;
            } else {
                // check if 2 nearest by lower_bound can be merged together
                if ranges[i + 1].lower_bound >= ranges[i].lower_bound
                    && ranges[i + 1].lower_bound <= ranges[i].upper_bound
                {
                    next_ranges.push(Range {
                        lower_bound: ranges[i].lower_bound,
                        upper_bound: ranges[i].upper_bound.max(ranges[i + 1].upper_bound),
                    });
                    did_change = true;
                    skip_next = true;
                } else {
                    next_ranges.push(ranges[i].clone());
                }
            }
        }
        if !skip_next {
            next_ranges.push(ranges[ranges.len() - 1].clone());
        }

        ranges = next_ranges;
    }

    let count: usize = ranges
        .iter()
        .map(|r| r.upper_bound - r.lower_bound + 1)
        .sum();

    assert_eq!(count, 338189277144473);
}
