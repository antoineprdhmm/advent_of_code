use crate::read_input;

fn parse_input() -> (Vec<[usize; 5]>, Vec<[usize; 5]>) {
    let mut schematics = Vec::new();
    let mut buffer = Vec::new();
    read_input(2024, 25)
        .unwrap()
        .map_while(Result::ok)
        .for_each(|line| {
            if line.is_empty() {
                schematics.push(std::mem::take(&mut buffer));
            } else {
                buffer.push(line);
            }
        });
    schematics.push(std::mem::take(&mut buffer));

    println!("{:?}", schematics);
    let (locks, keys): (Vec<Vec<String>>, Vec<Vec<String>>) = schematics
        .into_iter()
        .partition(|lines| lines.first().is_some_and(|s| s == "#####"));

    let locks: Vec<[usize; 5]> = locks
        .iter()
        .map(|lock| {
            let mut counts = [0; 5];

            for line in lock {
                for (i, c) in line.chars().enumerate() {
                    if c == '#' {
                        counts[i] += 1;
                    }
                }
            }

            counts
        })
        .map(|counts| counts.map(|c| c - 1))
        .collect();

    let keys: Vec<[usize; 5]> = keys
        .iter()
        .map(|lock| {
            let mut counts = [0; 5];

            for line in lock {
                for (i, c) in line.chars().enumerate() {
                    if c == '#' {
                        counts[i] += 1;
                    }
                }
            }

            counts
        })
        .map(|counts| counts.map(|c| c - 1))
        .collect();

    (locks, keys)
}

pub fn run_part_1() {
    let (locks, keys) = parse_input();

    let mut count = 0;

    for lock in &locks {
        for key in &keys {
            if lock[0] + key[0] <= 5
                && lock[1] + key[1] <= 5
                && lock[2] + key[2] <= 5
                && lock[3] + key[3] <= 5
                && lock[4] + key[4] <= 5
            {
                count += 1;
            }
        }
    }

    println!("{:?}", count);
}
