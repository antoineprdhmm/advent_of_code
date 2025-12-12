use std::collections::HashSet;

use crate::read_input;

struct Map(Vec<Vec<usize>>);

impl Map {
    fn get_next_elevation_positions(
        &self,
        position: (usize, usize),
        elevation: usize,
    ) -> Vec<(usize, usize)> {
        let mut next_positions = Vec::new();

        // top
        if position.0 > 0 && self.0[position.0 - 1][position.1] == elevation {
            next_positions.push((position.0 - 1, position.1));
        }

        // bottom
        if position.0 < (self.0.len() - 1) && self.0[position.0 + 1][position.1] == elevation {
            next_positions.push((position.0 + 1, position.1));
        }

        // left
        if position.1 > 0 && self.0[position.0][position.1 - 1] == elevation {
            next_positions.push((position.0, position.1 - 1));
        }

        // right
        if position.1 < (self.0[0].len() - 1) && self.0[position.0][position.1 + 1] == elevation {
            next_positions.push((position.0, position.1 + 1));
        }

        next_positions
    }

    fn get_trailhead_rating(&self, r: usize, c: usize) -> usize {
        let mut elevation = 0;
        let mut positions = vec![(r, c)];

        while !positions.is_empty() && elevation < 9 {
            elevation += 1;
            let mut next_positions = Vec::new();

            for position in positions {
                next_positions.extend(self.get_next_elevation_positions(position, elevation));
            }

            positions = next_positions;
        }

        positions.len()
    }

    fn get_trailhead_score(&self, r: usize, c: usize) -> usize {
        let mut elevation = 0;
        let mut positions = HashSet::from_iter(vec![(r, c)]);

        while !positions.is_empty() && elevation < 9 {
            elevation += 1;
            let mut next_positions = HashSet::new();

            for position in positions {
                next_positions.extend(self.get_next_elevation_positions(position, elevation));
            }

            positions = next_positions;
        }

        positions.len()
    }

    pub fn get_trailheads_scores(&self) -> usize {
        let mut score = 0;

        for r in 0..self.0.len() {
            for c in 0..self.0[r].len() {
                if self.0[r][c] == 0 {
                    score += self.get_trailhead_score(r, c);
                }
            }
        }

        score
    }

    pub fn get_trailheads_ratings(&self) -> usize {
        let mut rating = 0;

        for r in 0..self.0.len() {
            for c in 0..self.0[r].len() {
                if self.0[r][c] == 0 {
                    rating += self.get_trailhead_rating(r, c);
                }
            }
        }

        rating
    }
}

fn read_map() -> Map {
    let grid = read_input(2024, 10)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).map(|n| n as usize).expect(""))
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    Map(grid)
}

pub fn run_part_1() {
    let map = read_map();
    let score = map.get_trailheads_scores();
    println!("{score}");
}

pub fn run_part_2() {
    let map = read_map();
    let rating = map.get_trailheads_ratings();
    println!("{rating}");
}
