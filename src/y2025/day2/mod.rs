use crate::read_input;

struct IdRange {
    min: isize,
    max: isize,
}

fn read_id_ranges() -> Vec<IdRange> {
    read_input(2025, 2)
        .unwrap()
        .map_while(Result::ok)
        .last()
        .unwrap()
        .split(",")
        .map(|range_str| {
            let (min_str, max_str) = range_str.split_once("-").unwrap();
            let min: isize = min_str.parse().unwrap();
            let max: isize = max_str.parse().unwrap();

            IdRange { min, max }
        })
        .collect()
}

pub fn run_part_1() {
    let ranges = read_id_ranges();
    let mut invalid_ids_sum = 0;

    for range in ranges {
        for i in range.min..=range.max {
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

// Not the most efficient, but easy to implement
pub fn run_part_2() {
    let ranges = read_id_ranges();
    let mut invalid_ids_sum = 0;

    for range in ranges {
        for i in range.min..=range.max {
            let s = i.to_string();

            // The pattern should repeat at least 2 times and make the full `i`
            // So we can just check all possible chunks of same size, <= half the length of `i`
            // And see if they are all equal
            for j in 1..(s.len() / 2 + 1) {
                let s: Vec<String> = s
                    .chars()
                    .collect::<Vec<_>>()
                    .chunks(j)
                    .map(|chunk| chunk.iter().collect())
                    .collect();

                // If all are equal, we found an invalid id
                if s.iter().all(|x| x == &s[0]) {
                    invalid_ids_sum += i;
                    break;
                }
            }
        }
    }

    assert_eq!(invalid_ids_sum, 30260171216);
}
