use std::collections::{HashMap, HashSet};

use crate::read_input;

fn read_initial_secret_numbers() -> Vec<usize> {
    read_input(2024, 22)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

fn generate_next_secret(secret: usize) -> usize {
    let mut s1 = secret << 6;
    s1 ^= secret;
    s1 &= (1 << 24) - 1;

    let mut s2 = s1 >> 5;
    s2 ^= s1;
    s2 &= (1 << 24) - 1;

    let mut s3 = s2 << 11;
    s3 ^= s2;
    s3 &= (1 << 24) - 1;

    s3
}

fn generate_next_secrets(initial_secret: usize, nb: usize) -> usize {
    let mut secret = initial_secret;
    for _ in 0..nb {
        secret = generate_next_secret(secret);
    }
    secret
}

pub fn run_part_1() {
    let initial_secret_numbers = read_initial_secret_numbers();

    let sum = initial_secret_numbers
        .iter()
        .map(|secret| generate_next_secrets(*secret, 2000))
        .sum::<usize>();

    println!("{:?}", sum);
}

type Sequence = (isize, isize, isize, isize);

fn generate_buyer_sequences(initial_secret: usize) -> HashMap<Sequence, usize> {
    let mut sequences: HashMap<Sequence, usize> = HashMap::new();

    let mut secret = initial_secret;
    let mut previous_price = (secret % 10) as isize;

    let mut changes = [0isize; 4];

    // valid sequence contains 4 digits, so simply compute the changes for the first 3 prices
    for change in changes.iter_mut().take(3) {
        secret = generate_next_secret(secret);
        let digit = (secret % 10) as isize;
        *change = digit - previous_price;
        previous_price = digit;
    }

    for _ in 4..2000 {
        secret = generate_next_secret(secret);
        let digit = (secret % 10) as isize;
        changes[3] = digit - previous_price;
        previous_price = digit;

        let sequence = (changes[0], changes[1], changes[2], changes[3]);
        // keep the price for the first occurence of each sequence
        sequences
            .entry(sequence)
            .or_insert_with(|| previous_price.try_into().unwrap());

        // sliding window, only keep the last 3 prices
        // shift values for next loop iteration
        // changes[3] will be set with next price on next loop iteration
        changes[0] = changes[1];
        changes[1] = changes[2];
        changes[2] = changes[3];
    }

    sequences
}

pub fn run_part_2() {
    let buyers_initial_secret_numbers = read_initial_secret_numbers();

    let mut unique_sequences = HashSet::<Sequence>::new();
    let mut buyers_sequences = Vec::new();

    buyers_initial_secret_numbers.iter().for_each(|secret| {
        let sequences = generate_buyer_sequences(*secret);
        unique_sequences.extend(sequences.keys());
        buyers_sequences.push(sequences);
    });

    let max_bananas = unique_sequences
        .iter()
        .map(|sequence| {
            buyers_sequences
                .iter()
                .map(|buyer_sequences| buyer_sequences.get(sequence).unwrap_or(&0))
                .sum::<usize>()
        })
        .max();

    println!("{:?}", max_bananas);
}
