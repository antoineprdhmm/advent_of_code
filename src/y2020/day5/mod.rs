use crate::read_input;

fn read_boarding_passes() -> Vec<Vec<char>> {
    read_input(2020, 5)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| line.chars().collect())
        .collect()
}

fn compute_col(boarding_passes: &[char]) -> usize {
    let mut col = 0;
    let mut half_size = 4;

    for boarding_pass in boarding_passes.iter().take(10).skip(7) {
        if *boarding_pass == 'R' {
            col += half_size;
        }

        half_size /= 2;
    }

    col
}

fn compute_row(boarding_passes: &[char]) -> usize {
    let mut row = 0;
    let mut half_size = 64;

    for boarding_pass in boarding_passes.iter().take(7) {
        if *boarding_pass == 'B' {
            row += half_size;
        }

        half_size /= 2;
    }

    row
}

fn compute_seat_id(boarding_pass: &[char]) -> usize {
    compute_row(boarding_pass) * 8 + compute_col(boarding_pass)
}

pub fn run_part_1() {
    let highest_seat_id = read_boarding_passes()
        .iter()
        .map(|boarding_pass| compute_seat_id(boarding_pass))
        .max();

    assert_eq!(highest_seat_id, Some(970));
}

pub fn run_part_2() {
    let mut seat_ids = read_boarding_passes()
        .iter()
        .map(|boarding_pass| compute_seat_id(boarding_pass))
        .collect::<Vec<usize>>();

    seat_ids.sort();

    let mut my_seat_id = 0;
    for i in 1..seat_ids.len() {
        if seat_ids[i] - seat_ids[i - 1] == 2 {
            my_seat_id = seat_ids[i] - 1;
            break;
        }
    }
    assert_eq!(my_seat_id, 587);
}
