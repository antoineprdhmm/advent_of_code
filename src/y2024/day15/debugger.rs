use std::{fmt::Debug, io::Write};

use super::warehouse::Move;

#[allow(dead_code)]
const DEBUG_FILE: &str = "src/y2024/day15/debug_output.txt";

impl Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.0, self.1) {
            (0, -1) => write!(f, "LEFT"),
            (0, 1) => write!(f, "RIGHT"),
            (1, 0) => write!(f, "DOWN"),
            (-1, 0) => write!(f, "UP"),
            (x, y) => write!(f, "{x} {y}"),
        }
    }
}

#[derive(Default)]
pub struct WarehouseDebugger {
    pub is_activated: bool,
    pub debug_file: Option<std::fs::File>,
}

impl WarehouseDebugger {
    #[allow(dead_code)]
    pub fn activate_debug(&mut self) {
        self.is_activated = true;
        self.debug_file =
            Some(std::fs::File::create(DEBUG_FILE).expect("Failed to create debug file"));
    }

    pub fn debug_move(&mut self, r#move: &Move, new_state: &str) {
        if let Some(file) = &mut self.debug_file {
            writeln!(file, "Move {:?}", r#move).expect("Failed to write move");
            writeln!(file, "{}", new_state).expect("Failed to write grid");
            writeln!(file, "-----------------------").expect("Failed to write separator");
            writeln!(file).expect("Failed to write final newline");
        }
    }

    pub fn debug_initial_state(&mut self, state: &str) {
        if let Some(file) = &mut self.debug_file {
            writeln!(file, "{}", state).expect("Failed to write grid");
            writeln!(file, "-----------------------").expect("Failed to write separator");
            writeln!(file).expect("Failed to write final newline");
        }
    }
}
