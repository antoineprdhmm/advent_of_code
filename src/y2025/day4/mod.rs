use crate::read_input;

fn read_is_roll_grid() -> Vec<Vec<bool>> {
    read_input(2025, 4)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect()
}

pub fn run_part_1() {
    let grid = read_is_roll_grid();
    let mut count = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] {
                let mut adjacent_rolls = 0;

                // top
                if i > 0 {
                    if grid[i - 1][j] {
                        adjacent_rolls += 1;
                    }
                    // top left
                    if j > 0 && grid[i - 1][j - 1] {
                        adjacent_rolls += 1;
                    }
                    // top right
                    if j < (grid[i].len() - 1) && grid[i - 1][j + 1] {
                        adjacent_rolls += 1;
                    }
                }

                // left
                if j > 0 && grid[i][j - 1] {
                    adjacent_rolls += 1;
                }

                // right
                if j < (grid[i].len() - 1) && grid[i][j + 1] {
                    adjacent_rolls += 1;
                }

                // bottom
                if i < (grid.len() - 1) {
                    if grid[i + 1][j] {
                        adjacent_rolls += 1;
                    }
                    // bottom left
                    if j > 0 && grid[i + 1][j - 1] {
                        adjacent_rolls += 1;
                    }
                    // bottom right
                    if j < (grid[i].len() - 1) && grid[i + 1][j + 1] {
                        adjacent_rolls += 1;
                    }
                }

                if adjacent_rolls < 4 {
                    count += 1;
                }
            }
        }
    }

    assert_eq!(count, 1569);
}

pub fn run_part_2() {
    let mut grid = read_is_roll_grid();
    let mut count = 0;

    // Continue while we are removing rolls
    let mut did_remove_roll = true;
    while did_remove_roll {
        let mut next_grid = grid.clone();

        did_remove_roll = false;

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] {
                    let mut adjacent_rolls = 0;

                    // top
                    if i > 0 {
                        if grid[i - 1][j] {
                            adjacent_rolls += 1;
                        }
                        // top left
                        if j > 0 && grid[i - 1][j - 1] {
                            adjacent_rolls += 1;
                        }
                        // top right
                        if j < (grid[i].len() - 1) && grid[i - 1][j + 1] {
                            adjacent_rolls += 1;
                        }
                    }

                    // left
                    if j > 0 && grid[i][j - 1] {
                        adjacent_rolls += 1;
                    }

                    // right
                    if j < (grid[i].len() - 1) && grid[i][j + 1] {
                        adjacent_rolls += 1;
                    }

                    // bottom
                    if i < (grid.len() - 1) {
                        if grid[i + 1][j] {
                            adjacent_rolls += 1;
                        }
                        // bottom left
                        if j > 0 && grid[i + 1][j - 1] {
                            adjacent_rolls += 1;
                        }
                        // bottom right
                        if j < (grid[i].len() - 1) && grid[i + 1][j + 1] {
                            adjacent_rolls += 1;
                        }
                    }

                    if adjacent_rolls < 4 {
                        count += 1;
                        next_grid[i][j] = false;
                        did_remove_roll = true;
                    }
                }
            }
        }

        // update the grid for next round, without the removed rolls
        grid = next_grid;
    }

    assert_eq!(count, 9280);
}
