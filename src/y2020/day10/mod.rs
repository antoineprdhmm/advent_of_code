use std::collections::HashMap;

use crate::read_input;

fn read_ratings() -> Vec<usize> {
    read_input(2020, 10)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| line.parse().unwrap())
        .collect()
}

fn count_jolt_diff(ratings: &[usize], diff: usize) -> usize {
    let mut count = 0;
    if ratings[0] == diff {
        count += 1;
    }
    for i in 0..(ratings.len() - 1) {
        if ratings[i + 1] - ratings[i] == diff {
            count += 1;
        }
    }

    count
}

fn count_arrangements(ratings: Vec<usize>) -> usize {
    let mut memo: HashMap<usize, usize> = HashMap::new();
    memo.insert(0, 1);

    for rating in ratings.clone() {
        let mut count = 0;

        if rating > 2 {
            if let Some(c) = memo.get(&(rating - 3)) {
                count += c;
            }
        }
        if rating > 1 {
            if let Some(c) = memo.get(&(rating - 2)) {
                count += c;
            }
        }
        if let Some(c) = memo.get(&(rating - 1)) {
            count += c;
        }

        memo.insert(rating, count);
    }

    *memo.get(&ratings[ratings.len() - 1]).unwrap()
}

pub fn run_part_1() {
    let mut ratings = read_ratings();
    ratings.sort();
    // +1 because there is a diff of 3 between the last rating and the device
    let result = count_jolt_diff(&ratings, 1) * (count_jolt_diff(&ratings, 3) + 1);
    assert_eq!(result, 3034);
}

pub fn run_part_2() {
    let mut ratings = read_ratings();
    ratings.sort();

    let mut extended_ratings = vec![];
    extended_ratings.append(&mut ratings);
    extended_ratings.push(extended_ratings[extended_ratings.len() - 1] + 3);

    let result = count_arrangements(extended_ratings);
    assert_eq!(result, 259172170858496);
}
