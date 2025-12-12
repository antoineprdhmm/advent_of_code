use std::collections::HashMap;

use lazy_static::lazy_static;

use super::dijkstra::keypad_dijkstra;

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Direction::Up => "^",
            Direction::Down => "v",
            Direction::Right => ">",
            Direction::Left => "<",
        }
    }
}

lazy_static! {
    pub static ref NUMERIC_KEYPAD: HashMap<&'static str, Vec<(Direction, &'static str)>> = {
        let mut m = HashMap::new();

        // Top row
        m.insert("7", vec![(Direction::Right, "8"), (Direction::Down, "4")]);
        m.insert("8", vec![(Direction::Left, "7"), (Direction::Right, "9"), (Direction::Down, "5")]);
        m.insert("9", vec![(Direction::Left, "8"), (Direction::Down, "6")]);

        // Middle row
        m.insert("4", vec![(Direction::Up, "7"), (Direction::Right, "5"), (Direction::Down, "1")]);
        m.insert("5", vec![(Direction::Up, "8"), (Direction::Left, "4"), (Direction::Right, "6"), (Direction::Down, "2")]);
        m.insert("6", vec![(Direction::Up, "9"), (Direction::Left, "5"), (Direction::Down, "3")]);

        // Bottom row
        m.insert("1", vec![(Direction::Up, "4"), (Direction::Right, "2")]);
        m.insert("2", vec![(Direction::Up, "5"), (Direction::Left, "1"), (Direction::Right, "3"), (Direction::Down, "0")]);
        m.insert("3", vec![(Direction::Up, "6"), (Direction::Left, "2"), (Direction::Down, "A")]);

        // Bottom-most row
        m.insert("0", vec![(Direction::Up, "2"), (Direction::Right, "A")]);
        m.insert("A", vec![(Direction::Up, "3"), (Direction::Left, "0")]);

        m
    };

    pub static ref DIRECTIONAL_KEYPAD: HashMap<&'static str, Vec<(Direction, &'static str)>> = {
        let mut m = HashMap::new();

        // Top row
        m.insert("^", vec![(Direction::Right, "A"), (Direction::Down, "v")]);
        m.insert("A", vec![(Direction::Left, "^"), (Direction::Down, ">")]);

        // Bottom row
        m.insert("<", vec![(Direction::Right, "v")]);
        m.insert("v", vec![(Direction::Up, "^"), (Direction::Left, "<"), (Direction::Right, ">")]);
        m.insert(">", vec![(Direction::Up, "A"), (Direction::Left, "v")]);

        m
    };
}

pub struct Keypad {
    keys: &'static HashMap<&'static str, Vec<(Direction, &'static str)>>,
}

impl Keypad {
    pub fn new(keys: &'static HashMap<&'static str, Vec<(Direction, &'static str)>>) -> Keypad {
        Keypad { keys }
    }

    pub fn find_keypad_shortest_sequences(&self, from: &str, to: &str) -> Vec<String> {
        let mut sequences = keypad_dijkstra(from, to, self.keys);
        // press A after each sequence
        // the sequence moves the arm from `from` to `to` but then need to push the key with `A`
        sequences.iter_mut().for_each(|s| s.push('A'));
        sequences
    }
}
