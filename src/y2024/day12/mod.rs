use crate::read_input;
use std::collections::HashSet;

struct Cell {
    label: char,
    visited: bool,
}

type Grid = Vec<Vec<Cell>>;

fn init_garden() -> Garden {
    let grid = read_input(2024, 12)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| {
            line.chars()
                .map(|label| Cell {
                    label,
                    visited: false,
                })
                .collect()
        })
        .collect();

    Garden::new(grid)
}

struct Garden {
    grid: Grid,
}

impl Garden {
    pub fn new(grid: Grid) -> Self {
        Garden { grid }
    }

    fn get_region_fences_price(grid: &mut Grid, r: usize, c: usize) -> usize {
        let mut area = 0;
        let mut perimeter = 0;
        let mut queue = HashSet::new();
        queue.insert((r, c));
        let label = grid[r][c].label;

        while !queue.is_empty() {
            let mut new_queue = HashSet::new();

            for cell in queue {
                area += 1;
                grid[cell.0][cell.1].visited = true;

                if cell.0 == 0 {
                    perimeter += 1;
                }
                if cell.1 == 0 {
                    perimeter += 1;
                }
                if cell.0 + 1 == grid.len() {
                    perimeter += 1;
                }
                if cell.1 + 1 == grid[0].len() {
                    perimeter += 1;
                }

                let adjacent = [
                    (cell.0.wrapping_sub(1), cell.1), // up
                    (cell.0 + 1, cell.1),             // down
                    (cell.0, cell.1.wrapping_sub(1)), // left
                    (cell.0, cell.1 + 1),             // right
                ];

                for &(nr, nc) in adjacent.iter() {
                    if nr < grid.len() && nc < grid[0].len() {
                        if grid[nr][nc].label != label {
                            perimeter += 1;
                        } else if !grid[nr][nc].visited {
                            new_queue.insert((nr, nc));
                        }
                    }
                }
            }

            queue = new_queue;
        }

        area * perimeter
    }

    fn get_region_new_fences_price(grid: &mut Grid, r: usize, c: usize) -> usize {
        let mut area = 0;
        let mut corners = 0;

        let mut region_cells_to_visit = HashSet::new();
        region_cells_to_visit.insert((r, c));

        let label = grid[r][c].label;

        while !region_cells_to_visit.is_empty() {
            let mut next_region_cells_to_visit = HashSet::new();

            for cell in region_cells_to_visit {
                area += 1;
                grid[cell.0][cell.1].visited = true;

                let top = if cell.0 > 0 {
                    Some((cell.0 - 1, cell.1))
                } else {
                    None
                };
                let right = if (cell.1 + 1) < grid[cell.0].len() {
                    Some((cell.0, cell.1 + 1))
                } else {
                    None
                };
                let down = if (cell.0 + 1) < grid.len() {
                    Some((cell.0 + 1, cell.1))
                } else {
                    None
                };
                let left = if cell.1 > 0 {
                    Some((cell.0, cell.1 - 1))
                } else {
                    None
                };

                let left_same_label = left.map_or(false, |pos| grid[pos.0][pos.1].label == label);
                let top_same_label = top.map_or(false, |pos| grid[pos.0][pos.1].label == label);
                let down_same_label = down.map_or(false, |pos| grid[pos.0][pos.1].label == label);
                let right_same_label = right.map_or(false, |pos| grid[pos.0][pos.1].label == label);

                corners += [
                    // Inner corners
                    top_same_label
                        && right_same_label
                        && grid[cell.0 - 1][cell.1 + 1].label != label,
                    top_same_label
                        && left_same_label
                        && grid[cell.0 - 1][cell.1 - 1].label != label,
                    down_same_label
                        && right_same_label
                        && grid[cell.0 + 1][cell.1 + 1].label != label,
                    down_same_label
                        && left_same_label
                        && grid[cell.0 + 1][cell.1 - 1].label != label,
                    // Outer corners
                    !right_same_label && !top_same_label,
                    !right_same_label && !down_same_label,
                    !left_same_label && !top_same_label,
                    !left_same_label && !down_same_label,
                ]
                .iter()
                .filter(|v| **v)
                .collect::<Vec<_>>()
                .len();

                let adjacent = [top, right, down, left]
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>();

                for &(nr, nc) in adjacent.iter() {
                    if nr < grid.len()
                        && nc < grid[0].len()
                        && grid[nr][nc].label == label
                        && !grid[nr][nc].visited
                    {
                        next_region_cells_to_visit.insert((nr, nc));
                    }
                }
            }

            region_cells_to_visit = next_region_cells_to_visit;
        }

        area * corners
    }

    pub fn get_fences_price(&mut self) -> usize {
        let mut price = 0;

        let rows = self.grid.len();
        let cols = self.grid[0].len();

        for r in 0..rows {
            for c in 0..cols {
                if !self.grid[r][c].visited {
                    price += Self::get_region_fences_price(&mut self.grid, r, c);
                }
            }
        }

        price
    }

    pub fn get_new_fences_price(&mut self) -> usize {
        let mut price = 0;

        let rows = self.grid.len();
        let cols = self.grid[0].len();

        for r in 0..rows {
            for c in 0..cols {
                if !self.grid[r][c].visited {
                    price += Self::get_region_new_fences_price(&mut self.grid, r, c);
                }
            }
        }

        price
    }
}

pub fn run_part_1() {
    let mut garden = init_garden();
    println!("{}", garden.get_fences_price());
}

pub fn run_part_2() {
    let mut garden = init_garden();
    println!("{}", garden.get_new_fences_price());
}
