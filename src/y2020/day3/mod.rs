use crate::read_input;

struct Cell {
    is_tree: bool,
}

fn read_map() -> Vec<Vec<Cell>> {
    read_input(2020, 3)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| Cell { is_tree: c == '#' })
                .collect()
        })
        .collect()
}

fn count_trees_on_slope(map: &[Vec<Cell>], right_step: usize, bottom_step: usize) -> usize {
    let mut trees_counter: usize = 0;

    let width = map[0].len();
    let mut col: usize = 0;

    for row in (0..(map.len() - 1)).step_by(bottom_step) {
        col = (col + right_step) % width;

        if map[row + bottom_step][col].is_tree {
            trees_counter += 1;
        }
    }

    trees_counter
}

pub fn run_part_1() {
    let map = read_map();
    let count = count_trees_on_slope(&map, 3, 1);

    println!("Count = {}", count);
    assert_eq!(count, 252);
}

pub fn run_part_2() {
    let map = read_map();

    let right_1_bottom_1 = count_trees_on_slope(&map, 1, 1);
    let right_3_bottom_1 = count_trees_on_slope(&map, 3, 1);
    let right_5_bottom_1 = count_trees_on_slope(&map, 5, 1);
    let right_7_bottom_1 = count_trees_on_slope(&map, 7, 1);
    let right_1_bottom_2 = count_trees_on_slope(&map, 1, 2);

    let count = right_1_bottom_1
        * right_3_bottom_1
        * right_5_bottom_1
        * right_7_bottom_1
        * right_1_bottom_2;

    println!("Count = {}", count);
    assert_eq!(count, 2608962048);
}
