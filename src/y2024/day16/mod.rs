use lazy_static::lazy_static;
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::read_input;

const ROTATION_COST: usize = 1000;
const FORWARD_COST: usize = 1;

type Maze = Vec<Vec<char>>;

// Represents a candidate position with its direction and score for Dijkstra
#[derive(Eq, PartialEq, Debug)]
struct TileCandidateScore {
    position: Position,
    direction: Direction,
    score: usize,
}

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
struct Direction {
    row: isize,
    col: isize,
}

impl Ord for TileCandidateScore {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .score
            .cmp(&self.score)
            .then_with(|| self.position.row.cmp(&other.position.row))
            .then_with(|| self.position.col.cmp(&other.position.col))
            .then_with(|| self.direction.row.cmp(&other.direction.row))
            .then_with(|| self.direction.col.cmp(&other.direction.col))
    }
}
impl PartialOrd for TileCandidateScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

const EAST: Direction = Direction { row: 0, col: 1 };
const WEST: Direction = Direction { row: 0, col: -1 };
const SOUTH: Direction = Direction { row: 1, col: 0 };
const NORTH: Direction = Direction { row: -1, col: 0 };

lazy_static! {
    static ref ROTATE_CLOCKWISE: HashMap<Direction, Direction> = {
        let mut m = HashMap::new();
        m.insert(NORTH, EAST);
        m.insert(EAST, SOUTH);
        m.insert(SOUTH, WEST);
        m.insert(WEST, NORTH);
        m
    };
    static ref ROTATE_COUNTERCLOCKWISE: HashMap<Direction, Direction> = {
        let mut m = HashMap::new();
        m.insert(EAST, NORTH);
        m.insert(SOUTH, EAST);
        m.insert(WEST, SOUTH);
        m.insert(NORTH, WEST);
        m
    };
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn apply_direction(&self, direction: &Direction) -> Self {
        Position {
            row: (self.row as isize + direction.row) as usize,
            col: (self.col as isize + direction.col) as usize,
        }
    }
}

type SmallestScoresByTile = HashMap<(Position, Direction), usize>;

fn parse_maze() -> Maze {
    read_input(2024, 16)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn extract_start_end(maze: &Maze) -> (Position, Position) {
    let start = Position {
        row: maze.len() - 2,
        col: 1,
    };
    let end = Position {
        row: 1,
        col: maze[0].len() - 2,
    };

    (start, end)
}

fn extract_smallest_score_for_position(
    smallest_scores: &SmallestScoresByTile,
    position: &Position,
) -> (Direction, usize) {
    let (dir, score) = [EAST, NORTH, SOUTH, WEST]
        .iter()
        .filter_map(|dir| {
            smallest_scores
                .get(&(*position, *dir))
                .map(|dist| (dir, dist))
        })
        .min_by_key(|(_, dist)| *dist)
        .unwrap();

    (*dir, *score)
}

// returns the smallest score for each tile of the maze
fn dijkstra(maze: Maze, start: Position) -> SmallestScoresByTile {
    let start_tile = TileCandidateScore {
        position: start,
        direction: EAST,
        score: 0,
    };

    let mut queue = BinaryHeap::new();
    queue.push(start_tile);

    let mut smallest_scores: SmallestScoresByTile = HashMap::new();

    while let Some(candidate) = queue.pop() {
        // already get a better score to reach this position with this direction -> skip
        if smallest_scores.contains_key(&(candidate.position, candidate.direction)) {
            continue;
        }

        // this is the best score to reach this position with this direction (because min BinaryHeap)
        smallest_scores.insert((candidate.position, candidate.direction), candidate.score);

        let next_position = candidate.position.apply_direction(&candidate.direction);

        if maze[next_position.row][next_position.col] != '#' {
            queue.push(TileCandidateScore {
                position: next_position,
                direction: candidate.direction,
                score: candidate.score + FORWARD_COST,
            });
        }

        queue.push(TileCandidateScore {
            position: candidate.position,
            direction: *ROTATE_CLOCKWISE.get(&candidate.direction).unwrap(),
            score: candidate.score + ROTATION_COST,
        });
        queue.push(TileCandidateScore {
            position: candidate.position,
            direction: *ROTATE_COUNTERCLOCKWISE.get(&candidate.direction).unwrap(),
            score: candidate.score + ROTATION_COST,
        });
    }

    smallest_scores
}

fn calculate_rotations(from_direction: Direction, to_direction: Direction) -> usize {
    [&*ROTATE_CLOCKWISE, &*ROTATE_COUNTERCLOCKWISE]
        .iter()
        .map(|rotate_map| {
            let mut rotations = 0;
            let mut current = from_direction;
            while current != to_direction {
                current = rotate_map[&current];
                rotations += 1;
            }
            rotations
        })
        .min()
        .unwrap()
}

// With dijkstra, we have the smallest score for each direction for each tile.
// Start at the end position, and go through the tiles backward following the best paths
// to discover all tiles beloging to a best path
fn backward_tracing(smallest_scores: &SmallestScoresByTile, end: &Position) -> HashSet<Position> {
    let (smallest_direction, smallest_score) =
        extract_smallest_score_for_position(smallest_scores, end);

    let mut tiles: HashSet<Position> = HashSet::new();
    let mut queue = [(*end, (smallest_direction, smallest_score))].to_vec();

    while !queue.is_empty() {
        let mut next_queue = Vec::new();

        for (tile, (smallest_direction, smallest_score)) in queue {
            tiles.insert(tile);

            // the idea is to find where we could have come from to get the score of `tile`

            // The smallest score of `tile` has been obtained using one of the 4 directions
            // -> find the predecessor tile based on this direction
            let opposite_dir = ROTATE_CLOCKWISE[&ROTATE_CLOCKWISE[&smallest_direction]];
            let predecessor_pos = tile.apply_direction(&opposite_dir);

            let predecessor_entries = [
                (smallest_scores.get(&(predecessor_pos, NORTH)), NORTH),
                (smallest_scores.get(&(predecessor_pos, SOUTH)), SOUTH),
                (smallest_scores.get(&(predecessor_pos, EAST)), EAST),
                (smallest_scores.get(&(predecessor_pos, WEST)), WEST),
            ]
            .to_vec();

            // Check the entries of this predecessor tile
            // to find which ones are valid entries for smallest score paths
            for (entry_score, entry_direction) in predecessor_entries {
                if let Some(score) = entry_score {
                    // find the number of rotations needed for the entry to be in the same direction as `tile`
                    let rotations = calculate_rotations(entry_direction, smallest_direction);

                    if score + rotations * ROTATION_COST + FORWARD_COST == smallest_score {
                        // the entry tile belongs to a best path
                        next_queue.push((predecessor_pos, (entry_direction, *score)));
                    }
                }
            }
        }

        queue = next_queue;
    }

    tiles
}

pub fn run_part_1() {
    let maze = parse_maze();
    let (start, end) = extract_start_end(&maze);

    let smallest_scores = dijkstra(maze, start);
    let (_, min_distance) = extract_smallest_score_for_position(&smallest_scores, &end);

    println!("{:?}", min_distance);
}

pub fn run_part_2() {
    let maze = parse_maze();
    let (start, end) = extract_start_end(&maze);

    let smallest_scores = dijkstra(maze, start);
    let best_paths_tiles = backward_tracing(&smallest_scores, &end);

    println!("{:?}", best_paths_tiles.len());
}
