use crate::read_input;

const MIN_LEVEL_DIFF: i32 = 1;
const MAX_LEVEL_DIFF: i32 = 3;

fn is_sequence_safe(levels: &[i32]) -> bool {
    let is_descending = levels[0] > levels[1];

    levels.windows(2).all(|window| {
        let diff = if is_descending {
            window[0] - window[1]
        } else {
            window[1] - window[0]
        };

        (MIN_LEVEL_DIFF..=MAX_LEVEL_DIFF).contains(&diff)
    })
}

fn is_sequence_safe_with_tolerate_single_bad_level(levels: &[i32]) -> bool {
    if is_sequence_safe(levels) {
        return true;
    }

    (0..levels.len()).any(|skip_idx| {
        let filtered: Vec<_> = levels
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != skip_idx)
            .map(|(_, v)| *v)
            .collect();
        is_sequence_safe(&filtered)
    })
}

pub fn run_part_1() {
    let lines = read_input(2024, 2).unwrap();

    let safe_count = lines
        .map_while(Result::ok)
        .map(|line| {
            let levels = line
                .split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            is_sequence_safe(&levels)
        })
        .filter(|&safe| safe)
        .count();

    println!("{}", safe_count);
}

pub fn run_part_2() {
    let lines = read_input(2024, 2).unwrap();

    let safe_count = lines
        .map_while(Result::ok)
        .map(|line| {
            let levels = line
                .split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            is_sequence_safe_with_tolerate_single_bad_level(&levels)
        })
        .filter(|&safe| safe)
        .count();

    println!("{}", safe_count);
}
