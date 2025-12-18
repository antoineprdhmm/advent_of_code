use std::ops::RangeInclusive;

use crate::read_input;

fn read_id_ranges() -> Vec<RangeInclusive<isize>> {
    read_input(2025, 2)
        .unwrap()
        .map_while(Result::ok)
        .last()
        .unwrap()
        .split(",")
        .map(|range_str| {
            let (min, max) = range_str.split_once("-").unwrap();
            min.parse().unwrap()..=max.parse().unwrap()
        })
        .collect()
}

pub fn run_part_1() {
    let ranges = read_id_ranges();
    let mut invalid_ids_sum = 0;

    for range in ranges {
        for i in range {
            let length = i.ilog10() + 1;

            if length % 2 == 0 {
                let half = length / 2;
                let divisor = 10_isize.pow(half);

                if i / divisor == i % divisor {
                    invalid_ids_sum += i;
                }
            }
        }
    }

    assert_eq!(invalid_ids_sum, 20223751480);
}

pub fn run_part_2() {
    let ranges = read_id_ranges();
    let mut invalid_ids_sum = 0;

    for range in ranges {
        for i in range {
            let s = i.to_string();

            // The pattern should repeat at least 2 times and make the full `i`
            // So we can just check all possible chunks of same size, <= half the length of `i`
            // And see if they are all equal
            for pattern_len in 1..(s.len() / 2 + 1) {
                if s.len() % pattern_len != 0 {
                    continue; // Pattern must divide evenly
                }

                let pattern = &s[..pattern_len];
                if s.as_bytes()
                    .chunks(pattern_len)
                    .all(|chunk| chunk == pattern.as_bytes())
                {
                    invalid_ids_sum += i;
                    break;
                }
            }
        }
    }

    assert_eq!(invalid_ids_sum, 30260171216);
}
