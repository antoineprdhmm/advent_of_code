use crate::read_input;

#[derive(PartialEq, Debug)]
enum Direction {
    Left,
    Right,
}

struct Rotation {
    direction: Direction,
    steps: isize,
}

fn read_rotations() -> Vec<Rotation> {
    read_input(2025, 1)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| {
            let (direction, steps) = line.split_at(1);
            Rotation {
                direction: if direction == "L" {
                    Direction::Left
                } else {
                    Direction::Right
                },
                steps: steps.parse::<isize>().unwrap(),
            }
        })
        .collect()
}

pub fn run_part_1() {
    let rotations = read_rotations();

    let mut count_0 = 0;
    let mut current_position = 50;

    for rotation in rotations {
        if rotation.direction == Direction::Left {
            let new_pos = (current_position as i32 - rotation.steps as i32) % 100;
            current_position = if new_pos < 0 { new_pos + 100 } else { new_pos };
        } else {
            current_position = (current_position as i32 + rotation.steps as i32) % 100;
        }

        if current_position == 0 {
            count_0 += 1;
        }
    }

    assert_eq!(count_0, 1129);
}

pub fn run_part_2() {
    let rotations = read_rotations();

    let mut count_0 = 0;
    let mut current_position = 50;

    for rotation in rotations {
        count_0 += rotation.steps / 100;
        let remaining_steps = rotation.steps % 100;

        match rotation.direction {
            Direction::Left => {
                let new_position_raw = current_position - remaining_steps;
                let new_position = if new_position_raw < 0 {
                    new_position_raw + 100
                } else {
                    new_position_raw
                };

                if new_position_raw < 0 && current_position > 0 {
                    count_0 += 1;
                }
                if new_position == 0 && current_position != 0 {
                    count_0 += 1;
                }

                current_position = new_position;
            }
            Direction::Right => {
                let new_position_raw = current_position + remaining_steps;
                let new_position = if new_position_raw > 99 {
                    new_position_raw - 100
                } else {
                    new_position_raw
                };

                if new_position_raw > 99 {
                    count_0 += 1;
                }

                current_position = new_position;
            }
        }
    }

    assert_eq!(count_0, 6638);
}
