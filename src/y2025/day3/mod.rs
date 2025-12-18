use crate::read_input;

fn read_banks() -> Vec<Vec<usize>> {
    read_input(2025, 3)
        .unwrap()
        .map_while(Result::ok)
        .map(|row| {
            row.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

pub fn run_part_1() {
    let joltage: usize = read_banks()
        .iter()
        .map(|bank| {
            let (mut a, mut b) = (bank[0], bank[1]);

            for i in 2..bank.len() {
                if b > a {
                    a = b;
                    b = bank[i];
                } else {
                    if bank[i] > b {
                        b = bank[i];
                    }
                }
            }

            a * 10 + b
        })
        .sum();

    assert_eq!(joltage, 16812);
}

pub fn run_part_2() {
    let joltage: usize = read_banks()
        .iter()
        .map(|bank| {
            let mut biggest: Vec<usize> = bank.iter().take(12).copied().collect();

            for i in biggest.len()..bank.len() {
                let mut shifted = false;
                let last_index = biggest.len() - 1;
                for j in 0..(last_index) {
                    if biggest[j] < biggest[j + 1] {
                        // shift
                        for k in j..(last_index) {
                            biggest[k] = biggest[k + 1];
                        }
                        // put new discovered value at the end after the shift
                        biggest[last_index] = bank[i];
                        shifted = true;
                        break;
                    }
                }
                // if not shifted, try to replace last value if it's bigger
                if !shifted && bank[i] > biggest[last_index] {
                    biggest[last_index] = bank[i];
                }
            }

            biggest.iter().fold(0, |acc, &d| acc * 10 + d)
        })
        .sum();

    assert_eq!(joltage, 166345822896410);
}
