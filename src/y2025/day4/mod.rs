use crate::read_input;

fn read_is_roll_grid() -> Vec<Vec<bool>> {
    read_input(2025, 4)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect()
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn count_adjacent_rolls(grid: &[Vec<bool>], row: usize, col: usize) -> usize {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    DIRECTIONS
        .iter()
        .filter(|&&(dr, dc)| {
            let nr = row as isize + dr;
            let nc = col as isize + dc;
            nr >= 0 && nr < rows && nc >= 0 && nc < cols && grid[nr as usize][nc as usize]
        })
        .count()
}

pub fn run_part_1() {
    let grid = read_is_roll_grid();

    let count = (0..grid.len())
        .flat_map(|i| (0..grid[i].len()).map(move |j| (i, j)))
        .filter(|&(i, j)| grid[i][j] && count_adjacent_rolls(&grid, i, j) < 4)
        .count();

    assert_eq!(count, 1569);
}

pub fn run_part_2() {
    let mut grid = read_is_roll_grid();
    let mut count = 0;

    loop {
        // Find all cells to remove this round
        let to_remove: Vec<(usize, usize)> = (0..grid.len())
            .flat_map(|i| (0..grid[i].len()).map(move |j| (i, j)))
            .filter(|&(i, j)| grid[i][j] && count_adjacent_rolls(&grid, i, j) < 4)
            .collect();

        if to_remove.is_empty() {
            break;
        }

        count += to_remove.len();
        for (i, j) in to_remove {
            grid[i][j] = false;
        }
    }

    assert_eq!(count, 9280);
}
