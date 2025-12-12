use crate::read_input;
use std::collections::{BinaryHeap, HashMap, HashSet};

type Position = (usize, usize);

#[derive(Eq, PartialEq)]
struct DijkstraCandidate {
    position: Position,
    distance_from_start: usize,
}

impl Ord for DijkstraCandidate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .distance_from_start
            .cmp(&self.distance_from_start)
            .then_with(|| self.position.0.cmp(&other.position.0))
            .then_with(|| self.position.1.cmp(&other.position.1))
    }
}
impl PartialOrd for DijkstraCandidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
enum Cell {
    Wall,
    Track,
}

struct ReachableCell {
    position: Position,
    distance: usize,
}

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
const MIN_SAVE_FOR_QUALIFIED_CHEAT: usize = 100;
const PART_1_CHEAT_DISTANCE: usize = 2;
const PART_2_CHEAT_DISTANCE: usize = 20;

struct RaceCondition {
    racetrack: Vec<Vec<Cell>>,
    start: Position,
    end: Position,
    nb_rows: usize,
    nb_cols: usize,
}

impl RaceCondition {
    fn new(racetrack: Vec<Vec<Cell>>, start: Position, end: Position) -> Self {
        RaceCondition {
            nb_rows: racetrack.len(),
            nb_cols: racetrack[0].len(),
            racetrack,
            start,
            end,
        }
    }

    fn get_cells_within_manhattan_distance(
        &self,
        from: Position,
        max_distance: usize,
    ) -> Vec<ReachableCell> {
        let (from_row, from_col) = from;
        let mut reachable_cells = Vec::new();

        let search_bound_min_row = from_row.saturating_sub(max_distance);
        let search_bound_min_col = from_col.saturating_sub(max_distance);
        let search_bound_max_row = std::cmp::min(from_row + max_distance, self.nb_rows - 1);
        let search_bound_max_col = std::cmp::min(from_col + max_distance, self.nb_cols - 1);

        for target_row in search_bound_min_row..=search_bound_max_row {
            for target_col in search_bound_min_col..=search_bound_max_col {
                if self.racetrack[target_row][target_col] == Cell::Wall {
                    continue;
                }

                let raw_distance = if target_row > from_row {
                    target_row - from_row
                } else {
                    from_row - target_row
                };
                let col_distance = if target_col > from_col {
                    target_col - from_col
                } else {
                    from_col - target_col
                };
                let manhattan_dist = raw_distance + col_distance;

                if manhattan_dist <= max_distance {
                    reachable_cells.push(ReachableCell {
                        position: (target_row, target_col),
                        distance: manhattan_dist,
                    });
                }
            }
        }

        reachable_cells
    }

    fn count_qualified_cheats(
        &mut self,
        shortest_distance_without_cheat: usize,
        min_save: usize,
        cheat_distance: usize,
    ) -> usize {
        let shortest_from_start = self.dijkstra(self.start, self.end);
        let shortest_from_end = self.dijkstra(self.end, self.start);

        let mut qualified_cheats: HashSet<(Position, Position)> = HashSet::new();

        for r in 1..self.nb_rows - 1 {
            for c in 1..self.nb_cols - 1 {
                if self.racetrack[r][c] == Cell::Wall {
                    continue;
                }

                if let Some(shortest_from_start) = shortest_from_start.get(&(r, c)) {
                    let reachable_cells =
                        self.get_cells_within_manhattan_distance((r, c), cheat_distance);

                    for reachable_cell in reachable_cells {
                        if let Some(shortest_from_end) =
                            shortest_from_end.get(&reachable_cell.position)
                        {
                            let total =
                                shortest_from_start + reachable_cell.distance + shortest_from_end;

                            if total < shortest_distance_without_cheat
                                && shortest_distance_without_cheat - total >= min_save
                            {
                                qualified_cheats.insert(((r, c), reachable_cell.position));
                            }
                        }
                    }
                }
            }
        }

        qualified_cheats.len()
    }

    fn dijkstra(&self, from: Position, to: Position) -> HashMap<Position, usize> {
        let start = DijkstraCandidate {
            position: from,
            distance_from_start: 0,
        };

        let mut queue = BinaryHeap::new();
        queue.push(start);

        let mut shortest_distances: HashMap<Position, usize> = HashMap::new();

        while let Some(candidate) = queue.pop() {
            if shortest_distances.contains_key(&(candidate.position.0, candidate.position.1)) {
                continue;
            }

            shortest_distances.insert(
                (candidate.position.0, candidate.position.1),
                candidate.distance_from_start,
            );

            if to == candidate.position {
                return shortest_distances;
            }

            for direction in DIRECTIONS {
                let target = (
                    (candidate.position.0 as isize + direction.0) as usize,
                    (candidate.position.1 as isize + direction.1) as usize,
                );

                if target.0 < self.nb_rows
                    && target.1 < self.nb_cols
                    && self.racetrack[target.0][target.1] == Cell::Track
                {
                    queue.push(DijkstraCandidate {
                        position: target,
                        distance_from_start: candidate.distance_from_start + 1,
                    });
                }
            }
        }

        panic!("Failed to reach the end");
    }
}

fn init_race_condition() -> RaceCondition {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let racetrack = read_input(2024, 20)
        .unwrap()
        .map_while(Result::ok)
        .enumerate()
        .map(|(r, row)| {
            row.chars()
                .enumerate()
                .map(|(c, ch)| match ch {
                    '#' => Cell::Wall,
                    '.' => Cell::Track,
                    'S' => {
                        start = (r, c);
                        Cell::Track
                    }
                    'E' => {
                        end = (r, c);
                        Cell::Track
                    }
                    _ => panic!("Unexpected cell character"),
                })
                .collect()
        })
        .collect::<Vec<Vec<Cell>>>();

    RaceCondition::new(racetrack, start, end)
}

pub fn run(cheat_distance: usize) -> usize {
    let mut race_condition = init_race_condition();
    let shortest_distance_without_cheat = *race_condition
        .dijkstra(race_condition.start, race_condition.end)
        .get(&race_condition.end)
        .unwrap();

    race_condition.count_qualified_cheats(
        shortest_distance_without_cheat,
        MIN_SAVE_FOR_QUALIFIED_CHEAT,
        cheat_distance,
    )
}

pub fn run_part_1() {
    let nb_qualified_cheats = run(PART_1_CHEAT_DISTANCE);
    println!("Part 1: {nb_qualified_cheats}");
}

pub fn run_part_2() {
    let nb_qualified_cheats = run(PART_2_CHEAT_DISTANCE);
    println!("Part 2: {nb_qualified_cheats}");
}
