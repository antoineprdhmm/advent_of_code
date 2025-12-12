use crate::read_input;

type Grid = Vec<Vec<char>>;

const VALID_X_MAS_PATTERNS: [(char, char, char, char); 4] = [
    ('M', 'S', 'M', 'S'),
    ('S', 'S', 'M', 'M'),
    ('S', 'M', 'S', 'M'),
    ('M', 'M', 'S', 'S'),
];

fn is_x_mas_pattern(r: usize, c: usize, grid: &Grid) -> bool {
    let can_top = r > 0;
    let can_right = c < grid[r].len() - 1;
    let can_left = c > 0;
    let can_bottom = r < grid.len() - 1;

    if can_top && can_bottom && can_left && can_right {
        let corners = (
            grid[r - 1][c - 1],
            grid[r - 1][c + 1],
            grid[r + 1][c - 1],
            grid[r + 1][c + 1],
        );

        return VALID_X_MAS_PATTERNS.contains(&corners);
    }

    false
}

fn count_patterns(r: usize, c: usize, grid: &Grid) -> usize {
    let can_top = r >= 3;
    let can_right = c < grid[r].len() - 3;
    let can_left = c >= 3;
    let can_bottom = r < grid.len() - 3;

    let mut xmas_count = 0;

    let right =
        can_right && grid[r][c + 1] == 'M' && grid[r][c + 2] == 'A' && grid[r][c + 3] == 'S';
    if right {
        xmas_count += 1;
    }

    let left = can_left && grid[r][c - 1] == 'M' && grid[r][c - 2] == 'A' && grid[r][c - 3] == 'S';
    if left {
        xmas_count += 1;
    }

    let bottom =
        can_bottom && grid[r + 1][c] == 'M' && grid[r + 2][c] == 'A' && grid[r + 3][c] == 'S';
    if bottom {
        xmas_count += 1;
    }

    let top = can_top && grid[r - 1][c] == 'M' && grid[r - 2][c] == 'A' && grid[r - 3][c] == 'S';
    if top {
        xmas_count += 1;
    }

    let top_right = can_top
        && can_right
        && grid[r - 1][c + 1] == 'M'
        && grid[r - 2][c + 2] == 'A'
        && grid[r - 3][c + 3] == 'S';
    if top_right {
        xmas_count += 1;
    }

    let top_left = can_top
        && can_left
        && grid[r - 1][c - 1] == 'M'
        && grid[r - 2][c - 2] == 'A'
        && grid[r - 3][c - 3] == 'S';
    if top_left {
        xmas_count += 1;
    }

    let bottom_right = can_bottom
        && can_right
        && grid[r + 1][c + 1] == 'M'
        && grid[r + 2][c + 2] == 'A'
        && grid[r + 3][c + 3] == 'S';
    if bottom_right {
        xmas_count += 1;
    }

    let bottom_left = can_bottom
        && can_left
        && grid[r + 1][c - 1] == 'M'
        && grid[r + 2][c - 2] == 'A'
        && grid[r + 3][c - 3] == 'S';
    if bottom_left {
        xmas_count += 1;
    }

    xmas_count
}

pub fn run_part_1() {
    let lines = read_input(2024, 4).unwrap();

    let grid = lines
        .map_while(Result::ok)
        .map(|line| line.chars().collect())
        .collect::<Grid>();

    let mut xmas_count = 0;

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == 'X' {
                xmas_count += count_patterns(r, c, &grid);
            }
        }
    }

    println!("{xmas_count}");
}

pub fn run_part_2() {
    let lines = read_input(2024, 4).unwrap();

    let grid = lines
        .map_while(Result::ok)
        .map(|line| line.chars().collect())
        .collect::<Grid>();

    let mut x_mas_count = 0;

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == 'A' && is_x_mas_pattern(r, c, &grid) {
                x_mas_count += 1;
            }
        }
    }

    println!("{x_mas_count}");
}
