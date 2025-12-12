use crate::read_input;

const FLOOR: char = '.';
const OCCUPIED_SEAT: char = '#';
const EMPTY_SEAT: char = 'L';

fn read_seat_layout() -> Vec<Vec<char>> {
    read_input(2020, 11)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

fn count_occupied_adjacent_seats(seat_layout: &[Vec<char>], line_i: usize, col_i: usize) -> usize {
    let mut count = 0;

    // top line
    if line_i > 0 && col_i > 0 && seat_layout[line_i - 1][col_i - 1] == OCCUPIED_SEAT {
        count += 1;
    }
    if line_i > 0 && seat_layout[line_i - 1][col_i] == OCCUPIED_SEAT {
        count += 1;
    }
    if line_i > 0
        && col_i < (seat_layout[line_i].len() - 1)
        && seat_layout[line_i - 1][col_i + 1] == OCCUPIED_SEAT
    {
        count += 1;
    }

    // same line
    if col_i > 0 && seat_layout[line_i][col_i - 1] == OCCUPIED_SEAT {
        count += 1;
    }
    if col_i < (seat_layout[line_i].len() - 1) && seat_layout[line_i][col_i + 1] == OCCUPIED_SEAT {
        count += 1;
    }

    // bottom line
    if line_i < (seat_layout.len() - 1)
        && col_i > 0
        && seat_layout[line_i + 1][col_i - 1] == OCCUPIED_SEAT
    {
        count += 1;
    }
    if line_i < (seat_layout.len() - 1) && seat_layout[line_i + 1][col_i] == OCCUPIED_SEAT {
        count += 1;
    }
    if line_i < (seat_layout.len() - 1)
        && col_i < (seat_layout[line_i].len() - 1)
        && seat_layout[line_i + 1][col_i + 1] == OCCUPIED_SEAT
    {
        count += 1;
    }

    count
}

fn apply_rules_to_seat_layout<F>(
    seat_layout: &[Vec<char>],
    count_occupied_adjacent_seats_count_fn: F,
    occupied_adjacent_seats_count_threshold: usize,
) -> Vec<Vec<char>>
where
    F: Fn(&[Vec<char>], usize, usize) -> usize,
{
    let mut result_seat_layout: Vec<Vec<char>> = vec![];

    for line_i in 0..seat_layout.len() {
        let mut seat_layout_line: Vec<char> = vec![];

        for col_i in 0..seat_layout[line_i].len() {
            if seat_layout[line_i][col_i] == FLOOR {
                seat_layout_line.push(FLOOR);
                continue;
            }

            let occupied_adjacent_seats_count =
                count_occupied_adjacent_seats_count_fn(seat_layout, line_i, col_i);

            if seat_layout[line_i][col_i] == OCCUPIED_SEAT {
                if occupied_adjacent_seats_count >= occupied_adjacent_seats_count_threshold {
                    seat_layout_line.push(EMPTY_SEAT);
                } else {
                    seat_layout_line.push(OCCUPIED_SEAT);
                }
                continue;
            }

            if seat_layout[line_i][col_i] == EMPTY_SEAT {
                if occupied_adjacent_seats_count == 0 {
                    seat_layout_line.push(OCCUPIED_SEAT);
                } else {
                    seat_layout_line.push(EMPTY_SEAT);
                }
                continue;
            }

            panic!(
                "unknown char in seat layout: {:?}",
                seat_layout[line_i][col_i]
            );
        }

        result_seat_layout.push(seat_layout_line);
    }

    result_seat_layout
}

fn has_seat_layout_changed(old_seat_layout: &[Vec<char>], new_seat_layout: &[Vec<char>]) -> bool {
    for line_i in 0..old_seat_layout.len() {
        for col_i in 0..old_seat_layout[line_i].len() {
            if old_seat_layout[line_i][col_i] != new_seat_layout[line_i][col_i] {
                return true;
            }
        }
    }

    false
}

fn count_occupied_seats(seat_layout: &[Vec<char>]) -> usize {
    seat_layout
        .iter()
        .flatten()
        .filter(|&c| *c == OCCUPIED_SEAT)
        .count()
}

pub fn run_part_1() {
    let mut old_seat_layout = read_seat_layout();
    let mut new_seat_layout =
        apply_rules_to_seat_layout(&old_seat_layout, count_occupied_adjacent_seats, 4);

    while has_seat_layout_changed(&old_seat_layout, &new_seat_layout) {
        old_seat_layout = new_seat_layout;
        new_seat_layout =
            apply_rules_to_seat_layout(&old_seat_layout, count_occupied_adjacent_seats, 4);
    }

    let count = count_occupied_seats(&new_seat_layout);
    assert_eq!(count, 2204);
}

fn is_closest_top_left_seat_occupied(
    seat_layout: &[Vec<char>],
    line_i: usize,
    col_i: usize,
) -> bool {
    let mut next_line_i: isize = line_i as isize - 1;
    let mut next_col_i: isize = col_i as isize - 1;

    while next_line_i >= 0 && next_col_i >= 0 {
        let c = seat_layout[next_line_i as usize][next_col_i as usize];
        if c == OCCUPIED_SEAT {
            return true;
        }
        if c == EMPTY_SEAT {
            return false;
        }

        next_line_i -= 1;
        next_col_i -= 1;
    }

    false
}

fn is_closest_top_seat_occupied(seat_layout: &[Vec<char>], line_i: usize, col_i: usize) -> bool {
    let mut next_line_i: isize = line_i as isize - 1;

    while next_line_i >= 0 {
        let c = seat_layout[next_line_i as usize][col_i];
        if c == OCCUPIED_SEAT {
            return true;
        }
        if c == EMPTY_SEAT {
            return false;
        }

        next_line_i -= 1;
    }

    false
}

fn is_closest_top_right_seat_occupied(
    seat_layout: &[Vec<char>],
    line_i: usize,
    col_i: usize,
) -> bool {
    let mut next_line_i: isize = line_i as isize - 1;
    let mut next_col_i = col_i + 1;

    while next_line_i >= 0 && next_col_i < seat_layout[line_i].len() {
        let c = seat_layout[next_line_i as usize][next_col_i];
        if c == OCCUPIED_SEAT {
            return true;
        }
        if c == EMPTY_SEAT {
            return false;
        }

        next_line_i -= 1;
        next_col_i += 1;
    }

    false
}

fn is_closest_left_seat_occupied(seat_layout: &[Vec<char>], line_i: usize, col_i: usize) -> bool {
    let mut next_col_i: isize = col_i as isize - 1;

    while next_col_i >= 0 {
        let c = seat_layout[line_i][next_col_i as usize];
        if c == OCCUPIED_SEAT {
            return true;
        }
        if c == EMPTY_SEAT {
            return false;
        }

        next_col_i -= 1;
    }

    false
}

fn is_closest_right_seat_occupied(seat_layout: &[Vec<char>], line_i: usize, col_i: usize) -> bool {
    let mut next_col_i = col_i + 1;

    while next_col_i < seat_layout[line_i].len() {
        let c = seat_layout[line_i][next_col_i];
        if c == OCCUPIED_SEAT {
            return true;
        }
        if c == EMPTY_SEAT {
            return false;
        }

        next_col_i += 1;
    }

    false
}

fn is_closest_bottom_left_seat_occupied(
    seat_layout: &[Vec<char>],
    line_i: usize,
    col_i: usize,
) -> bool {
    let mut next_line_i = line_i + 1;
    let mut next_col_i: isize = col_i as isize - 1;

    while next_line_i < seat_layout.len() && next_col_i >= 0 {
        let c = seat_layout[next_line_i][next_col_i as usize];
        if c == OCCUPIED_SEAT {
            return true;
        }
        if c == EMPTY_SEAT {
            return false;
        }

        next_line_i += 1;
        next_col_i -= 1;
    }

    false
}

fn is_closest_bottom_seat_occupied(seat_layout: &[Vec<char>], line_i: usize, col_i: usize) -> bool {
    let mut next_line_i = line_i + 1;

    while next_line_i < seat_layout.len() {
        let c = seat_layout[next_line_i][col_i];
        if c == OCCUPIED_SEAT {
            return true;
        }
        if c == EMPTY_SEAT {
            return false;
        }

        next_line_i += 1;
    }

    false
}

fn is_closest_bottom_right_seat_occupied(
    seat_layout: &[Vec<char>],
    line_i: usize,
    col_i: usize,
) -> bool {
    let mut next_line_i = line_i + 1;
    let mut next_col_i = col_i + 1;

    while next_line_i < seat_layout.len() && next_col_i < seat_layout[line_i].len() {
        let c = seat_layout[next_line_i][next_col_i];
        if c == OCCUPIED_SEAT {
            return true;
        }
        if c == EMPTY_SEAT {
            return false;
        }

        next_line_i += 1;
        next_col_i += 1;
    }

    false
}

fn count_occupied_adjacent_seats_part_2(
    seat_layout: &[Vec<char>],
    line_i: usize,
    col_i: usize,
) -> usize {
    let mut count = 0;

    // top line
    if is_closest_top_left_seat_occupied(seat_layout, line_i, col_i) {
        count += 1;
    }
    if is_closest_top_seat_occupied(seat_layout, line_i, col_i) {
        count += 1;
    }
    if is_closest_top_right_seat_occupied(seat_layout, line_i, col_i) {
        count += 1;
    }

    // same line
    if is_closest_left_seat_occupied(seat_layout, line_i, col_i) {
        count += 1;
    }
    if is_closest_right_seat_occupied(seat_layout, line_i, col_i) {
        count += 1;
    }

    // bottom line
    if is_closest_bottom_left_seat_occupied(seat_layout, line_i, col_i) {
        count += 1;
    }
    if is_closest_bottom_seat_occupied(seat_layout, line_i, col_i) {
        count += 1;
    }
    if is_closest_bottom_right_seat_occupied(seat_layout, line_i, col_i) {
        count += 1;
    }

    count
}

pub fn run_part_2() {
    let mut old_seat_layout = read_seat_layout();
    let mut new_seat_layout =
        apply_rules_to_seat_layout(&old_seat_layout, count_occupied_adjacent_seats_part_2, 5);

    while has_seat_layout_changed(&old_seat_layout, &new_seat_layout) {
        old_seat_layout = new_seat_layout;
        new_seat_layout =
            apply_rules_to_seat_layout(&old_seat_layout, count_occupied_adjacent_seats_part_2, 5);
    }

    let count = count_occupied_seats(&new_seat_layout);
    assert_eq!(count, 1986);
}
