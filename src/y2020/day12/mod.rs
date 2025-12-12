use crate::read_input;

struct Instruction {
    action: char,
    value: usize,
}

fn read_instructions() -> Vec<Instruction> {
    read_input(2020, 12)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| {
            let (action_str, value_str) = line.split_at(1);

            Instruction {
                action: action_str.chars().next().unwrap(),
                value: value_str.parse::<usize>().unwrap(),
            }
        })
        .collect()
}

fn compute_new_direction(current_direction: char, degrees: isize) -> char {
    let directions = ['n', 'e', 's', 'w'];
    let current_direction_i = directions
        .iter()
        .position(|&d| d == current_direction)
        .unwrap();

    let i = if degrees > 0 {
        (degrees as usize) / 90
    } else {
        ((degrees + 360) as usize) / 90
    };

    let next_direction_i = (current_direction_i + i) % 4;
    directions[next_direction_i]
}

fn process_instructions(instructions: &[Instruction]) -> (isize, isize) {
    let mut x: isize = 0;
    let mut y: isize = 0;

    let mut direction: char = 'e';

    for instruction in instructions {
        if instruction.action == 'L' {
            direction = compute_new_direction(direction, -(instruction.value as isize));
            continue;
        }
        if instruction.action == 'R' {
            direction = compute_new_direction(direction, instruction.value as isize);
            continue;
        }

        if instruction.action == 'F' {
            if direction == 'e' {
                y += instruction.value as isize;
                continue;
            }
            if direction == 'w' {
                y -= instruction.value as isize;
                continue;
            }
            if direction == 'n' {
                x += instruction.value as isize;
                continue;
            }
            if direction == 's' {
                x -= instruction.value as isize;
                continue;
            }
        }

        if instruction.action == 'N' {
            x += instruction.value as isize;
            continue;
        }
        if instruction.action == 'S' {
            x -= instruction.value as isize;
            continue;
        }
        if instruction.action == 'E' {
            y += instruction.value as isize;
            continue;
        }
        if instruction.action == 'W' {
            y -= instruction.value as isize;
            continue;
        }
    }

    (x, y)
}

fn compute_new_waypoint((x, y): (isize, isize), degrees: isize) -> (isize, isize) {
    let mut d = degrees;
    if degrees < 0 {
        d = degrees + 360;
    }

    if d == 90 {
        return (-y, x);
    }

    if d == 180 {
        return (-x, -y);
    }

    (y, -x)
}

fn process_instructions_waypoint(instructions: &[Instruction]) -> (isize, isize) {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut w_x: isize = 1;
    let mut w_y: isize = 10;

    for instruction in instructions {
        if instruction.action == 'L' {
            let (new_w_x, new_w_y) =
                compute_new_waypoint((w_x, w_y), -(instruction.value as isize));
            w_x = new_w_x;
            w_y = new_w_y;
            continue;
        }
        if instruction.action == 'R' {
            let (new_w_x, new_w_y) = compute_new_waypoint((w_x, w_y), instruction.value as isize);
            w_x = new_w_x;
            w_y = new_w_y;
            continue;
        }

        if instruction.action == 'F' {
            x += w_x * instruction.value as isize;
            y += w_y * instruction.value as isize;
        }

        if instruction.action == 'N' {
            w_x += instruction.value as isize;
            continue;
        }
        if instruction.action == 'S' {
            w_x -= instruction.value as isize;
            continue;
        }
        if instruction.action == 'E' {
            w_y += instruction.value as isize;
            continue;
        }
        if instruction.action == 'W' {
            w_y -= instruction.value as isize;
            continue;
        }
    }

    (x, y)
}

pub fn run_part_1() {
    let instructions = read_instructions();
    let (x, y) = process_instructions(&instructions);
    let manhattan_distance = (x.abs() + y.abs()) as usize;
    assert_eq!(manhattan_distance, 508);
}

pub fn run_part_2() {
    let instructions = read_instructions();
    let (x, y) = process_instructions_waypoint(&instructions);
    let manhattan_distance = (x.abs() + y.abs()) as usize;
    assert_eq!(manhattan_distance, 30761);
}
