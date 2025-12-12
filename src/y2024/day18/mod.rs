use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::read_input;

#[derive(Clone)]
struct MemorySpace {
    is_corrupted: bool,
}

type Memory = Vec<Vec<MemorySpace>>;
type Coordinate = (usize, usize);

#[derive(PartialEq, Eq, Debug)]
struct PathFindingMemorySpace {
    coordinate: Coordinate,
    predecessor: Option<Coordinate>,
    score: usize,
}

impl Ord for PathFindingMemorySpace {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .score
            .cmp(&self.score)
            .then_with(|| self.coordinate.1.cmp(&other.coordinate.1))
            .then_with(|| self.coordinate.0.cmp(&other.coordinate.0))
    }
}

impl PartialOrd for PathFindingMemorySpace {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Computer {
    memory: Memory,
    fallen_bytes: Vec<Coordinate>,
}

impl Computer {
    pub fn new(memory_size: usize, fallen_bytes: Vec<Coordinate>) -> Self {
        Self {
            memory: vec![
                vec![
                    MemorySpace {
                        is_corrupted: false
                    };
                    memory_size
                ]
                .to_vec();
                memory_size
            ]
            .to_vec(),
            fallen_bytes,
        }
    }

    pub fn run_simulation(&mut self, nb_rounds: usize) {
        for i in 0..nb_rounds {
            self.memory[self.fallen_bytes[i].1][self.fallen_bytes[i].0].is_corrupted = true;
        }
    }

    fn extract_shortest_path(
        predecessors: &HashMap<Coordinate, Coordinate>,
    ) -> HashSet<Coordinate> {
        let mut coordinates = HashSet::new();

        let mut predecessor = Some((MEMORY_SIZE - 1, MEMORY_SIZE - 1));

        while let Some(p) = predecessor {
            coordinates.insert(p);
            predecessor = predecessors.get(&p).copied();
        }

        coordinates
    }

    pub fn run_simulation_until_blocked(&mut self, start_at: usize) -> Coordinate {
        for i in 0..start_at {
            self.memory[self.fallen_bytes[i].0][self.fallen_bytes[i].1].is_corrupted = true;
        }

        let mut shortest_path = self.find_shortest_path();
        let mut i = start_at;

        while i < self.fallen_bytes.len() {
            self.memory[self.fallen_bytes[i].0][self.fallen_bytes[i].1].is_corrupted = true;
            if let Some(path) = &shortest_path {
                if path.contains(&self.fallen_bytes[i]) {
                    shortest_path = self.find_shortest_path();

                    if shortest_path.is_none() {
                        return self.fallen_bytes[i];
                    }
                }
            }

            i += 1;
        }

        panic!("No blocking byte found");
    }

    pub fn find_shortest_path(&self) -> Option<HashSet<Coordinate>> {
        let start = PathFindingMemorySpace {
            coordinate: (0, 0),
            predecessor: None,
            score: 0,
        };
        let mut queue = BinaryHeap::new();
        queue.push(start);

        let mut smallest_scores: HashMap<Coordinate, usize> = HashMap::new();
        let mut predecessors: HashMap<Coordinate, Coordinate> = HashMap::new();

        while let Some(space) = queue.pop() {
            if smallest_scores.contains_key(&space.coordinate) {
                continue;
            }
            smallest_scores.insert(space.coordinate, space.score);

            if let Some(p) = space.predecessor {
                predecessors.insert(space.coordinate, p);
            }

            if space.coordinate == (MEMORY_SIZE - 1, MEMORY_SIZE - 1) {
                return Some(Self::extract_shortest_path(&predecessors));
            }

            let directions: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

            directions.iter().for_each(|direction| {
                let target = (
                    space.coordinate.0 as isize + direction.0,
                    space.coordinate.1 as isize + direction.1,
                );

                if target.0 < 0
                    || target.1 < 0
                    || target.0 >= MEMORY_SIZE as isize
                    || target.1 >= MEMORY_SIZE as isize
                {
                    return;
                }

                if self.memory[target.0 as usize][target.1 as usize].is_corrupted {
                    return;
                }

                queue.push(PathFindingMemorySpace {
                    coordinate: (target.0 as usize, target.1 as usize),
                    predecessor: Some(space.coordinate),
                    score: space.score + 1,
                })
            });
        }

        None
    }
}

fn build_computer() -> Computer {
    let input = read_input(2024, 18)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| {
            let (s_row, s_col) = line.split_once(",").unwrap();
            (
                s_row.parse::<usize>().unwrap(),
                s_col.parse::<usize>().unwrap(),
            )
        })
        .collect();

    Computer::new(MEMORY_SIZE, input)
}

const MEMORY_SIZE: usize = 71;
const SIMULATION_ROUNDS: usize = 1024;

pub fn run_part_1() {
    let mut computer = build_computer();
    computer.run_simulation(SIMULATION_ROUNDS);
    let shortest_path = computer.find_shortest_path();

    if let Some(coordinates) = shortest_path {
        println!("{}", coordinates.len() - 1);
    } else {
        println!("No path found");
    }
}

pub fn run_part_2() {
    let mut computer = build_computer();
    let blocking_byte = computer.run_simulation_until_blocked(SIMULATION_ROUNDS);
    println!("{:?}", blocking_byte);
}
