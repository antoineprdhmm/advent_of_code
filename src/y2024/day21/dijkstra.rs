use std::collections::{BinaryHeap, HashMap};

use super::keypad::Direction;

#[derive(Eq, PartialEq)]
struct DijkstraCandidate {
    key: String,
    sequence: String,
}

impl Ord for DijkstraCandidate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .sequence
            .len()
            .cmp(&self.sequence.len())
            .then_with(|| self.key.cmp(&other.key))
    }
}
impl PartialOrd for DijkstraCandidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn keypad_dijkstra(
    from: &str,
    to: &str,
    keypad: &HashMap<&'static str, Vec<(Direction, &'static str)>>,
) -> Vec<String> {
    let start = DijkstraCandidate {
        key: from.to_string(),
        sequence: "".to_string(),
    };

    let mut queue = BinaryHeap::new();
    queue.push(start);

    let mut shortest_sequences: HashMap<String, Vec<String>> = HashMap::new();

    while let Some(candidate) = queue.pop() {
        let sequences = shortest_sequences.entry(candidate.key.clone()).or_default();

        if !sequences.is_empty() && sequences[0].len() != candidate.sequence.len() {
            // already found a shorter sequence, skip this one
            continue;
        }

        sequences.push(candidate.sequence.clone());

        let directions = keypad.get(&candidate.key.as_str()).unwrap();

        for (dir, target_key) in directions.iter() {
            let mut candidate_sequence = candidate.sequence.clone();
            candidate_sequence.push_str(dir.as_str());
            queue.push(DijkstraCandidate {
                key: target_key.to_string(),
                sequence: candidate_sequence,
            });
        }
    }

    if let Some(sequences) = shortest_sequences.get(to) {
        sequences.clone()
    } else {
        vec![]
    }
}
