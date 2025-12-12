use std::fmt::{self, Debug};

use super::debugger::WarehouseDebugger;

#[derive(Clone, PartialEq, Eq)]
pub struct Move(pub isize, pub isize);

#[derive(Debug, Clone)]
pub struct Position(pub usize, pub usize);

impl Position {
    pub fn apply_move(position: &Self, r#move: &Move) -> Position {
        Position(
            (position.0 as isize + r#move.0) as usize,
            (position.1 as isize + r#move.1) as usize,
        )
    }
}

pub struct Robot {
    pub position: Position,
    pub moves: Vec<Move>,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum Cell {
    Wall,
    Box,
    Empty,

    // For Part 2:
    BoxLeft,
    BoxRight,
}

impl Move {
    pub const LEFT: Move = Move(0, -1);
    pub const RIGHT: Move = Move(0, 1);
    pub const UP: Move = Move(-1, 0);
    pub const DOWN: Move = Move(1, 0);
}

pub struct Warehouse {
    pub robot: Robot,
    pub grid: Vec<Vec<Cell>>,
    pub debugger: WarehouseDebugger,
}

impl fmt::Display for Warehouse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rows = self
            .grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| match cell {
                        Cell::Wall => "#",
                        Cell::Empty => ".",
                        Cell::Box => "O",
                        Cell::BoxRight => "]",
                        Cell::BoxLeft => "[",
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>();

        let mut rows_str = rows[self.robot.position.0].clone();
        rows_str.replace_range(self.robot.position.1..self.robot.position.1 + 1, "@");
        rows[self.robot.position.0] = rows_str;

        write!(f, "{}", rows.join("\n") + "\n")
    }
}

impl Warehouse {
    pub fn make_wide(&mut self) {
        self.grid = self
            .grid
            .iter()
            .map(|row| {
                row.iter()
                    .flat_map(|cell| match cell {
                        Cell::Wall => [Cell::Wall, Cell::Wall],
                        Cell::Empty => [Cell::Empty, Cell::Empty],
                        Cell::Box => [Cell::BoxLeft, Cell::BoxRight],
                        v => panic!("Unsupported CellKind making wharehouse wide {:?}", v),
                    })
                    .collect()
            })
            .collect();
        self.robot.position.1 *= 2;
    }

    pub fn compute_gps_coordinates_sum(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .map(|(row, cells)| {
                cells
                    .iter()
                    .enumerate()
                    .map(|(col, cell)| {
                        if *cell == Cell::Box || *cell == Cell::BoxLeft {
                            100 * row + col
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn try_move_wide_box(&mut self, box_pos: &Position, r#move: &Move, is_readonly: bool) -> bool {
        // When moving horizontally, it's similar as part 1
        if *r#move == Move::LEFT || *r#move == Move::RIGHT {
            let target_pos = Position::apply_move(&Position::apply_move(box_pos, r#move), r#move);

            let target_cell = self.grid[target_pos.0][target_pos.1].clone();
            return match target_cell {
                // Can't move if it's a wall
                Cell::Wall => false,
                // Can move if it's empty
                Cell::Empty => {
                    if *r#move == Move::LEFT {
                        self.grid[target_pos.0][target_pos.1] = Cell::BoxLeft;
                        let right_pos: Position = Position::apply_move(box_pos, r#move);
                        self.grid[right_pos.0][right_pos.1] = Cell::BoxRight;
                    } else {
                        self.grid[target_pos.0][target_pos.1] = Cell::BoxRight;
                        let left_pos: Position = Position::apply_move(box_pos, r#move);
                        self.grid[left_pos.0][left_pos.1] = Cell::BoxLeft;
                    }
                    self.grid[box_pos.0][box_pos.1] = Cell::Empty;
                    return true;
                }
                // If there is a box, need to move it first so cell becomes empty
                Cell::BoxLeft | Cell::BoxRight => {
                    if self.try_move_wide_box(&target_pos, r#move, is_readonly) {
                        if *r#move == Move::LEFT {
                            self.grid[target_pos.0][target_pos.1] = Cell::BoxLeft;
                            let right_pos: Position = Position::apply_move(box_pos, r#move);
                            self.grid[right_pos.0][right_pos.1] = Cell::BoxRight;
                        } else {
                            self.grid[target_pos.0][target_pos.1] = Cell::BoxRight;
                            let left_pos: Position = Position::apply_move(box_pos, r#move);
                            self.grid[left_pos.0][left_pos.1] = Cell::BoxLeft;
                        }
                        self.grid[box_pos.0][box_pos.1] = Cell::Empty;
                        return true;
                    }
                    false
                }
                v => panic!("Unsupported CellKind for part wide box {:?}", v),
            };
        }

        // When moving vertically, need to handle both sides of the box at the same time
        let (left_pos, right_pos) = if self.grid[box_pos.0][box_pos.1] == Cell::BoxLeft {
            (box_pos.clone(), Position::apply_move(box_pos, &Move::RIGHT))
        } else {
            (Position::apply_move(box_pos, &Move::LEFT), box_pos.clone())
        };
        let (left_target_pos, right_target_pos) = (
            Position::apply_move(&left_pos, r#move),
            Position::apply_move(&right_pos, r#move),
        );

        let (left_target, right_target) = (
            &self.grid[left_target_pos.0][left_target_pos.1].clone(),
            &self.grid[right_target_pos.0][right_target_pos.1].clone(),
        );

        // If at least one side face a wall, can't move
        if *left_target == Cell::Wall || *right_target == Cell::Wall {
            return false;
        }

        // If both target cells are empty, no blocker, can move there
        if *left_target == Cell::Empty && *right_target == Cell::Empty {
            if !is_readonly {
                self.move_box(&left_pos, &right_pos, &left_target_pos, &right_target_pos);
            }
            return true;
        }

        // 2 boxes to move
        // [][]
        //  []
        // Try readonly first to see if both can move, then do then move.
        // Else it could move one but not the other because it's blocked.
        if *left_target == Cell::BoxRight && *right_target == Cell::BoxLeft {
            if self.try_move_wide_box(&left_target_pos, r#move, true)
                && self.try_move_wide_box(&right_target_pos, r#move, true)
            {
                if !is_readonly {
                    self.try_move_wide_box(&left_target_pos, r#move, is_readonly);
                    self.try_move_wide_box(&right_target_pos, r#move, is_readonly);
                    self.move_box(&left_pos, &right_pos, &left_target_pos, &right_target_pos);
                }

                return true;
            }
            return false;
        }

        //   []
        //  []
        if *left_target == Cell::Empty
            && self.try_move_wide_box(&right_target_pos, r#move, is_readonly)
        {
            if !is_readonly {
                self.move_box(&left_pos, &right_pos, &left_target_pos, &right_target_pos);
            }
            return true;
        }

        // []
        //  []
        if *right_target == Cell::Empty
            && self.try_move_wide_box(&left_target_pos, r#move, is_readonly)
        {
            if !is_readonly {
                self.move_box(&left_pos, &right_pos, &left_target_pos, &right_target_pos);
            }
            return true;
        }

        //  []
        //  []
        if *right_target == Cell::BoxRight
            && *left_target == Cell::BoxLeft
            && self.try_move_wide_box(&left_target_pos, r#move, is_readonly)
        {
            if !is_readonly {
                self.move_box(&left_pos, &right_pos, &left_target_pos, &right_target_pos);
            }
            return true;
        }

        false
    }

    fn move_box(
        &mut self,
        left_pos: &Position,
        right_pos: &Position,
        left_target_pos: &Position,
        right_target_pos: &Position,
    ) {
        self.grid[left_target_pos.0][left_target_pos.1] = Cell::BoxLeft;
        self.grid[right_target_pos.0][right_target_pos.1] = Cell::BoxRight;
        self.grid[left_pos.0][left_pos.1] = Cell::Empty;
        self.grid[right_pos.0][right_pos.1] = Cell::Empty;
    }

    fn try_move_box(&mut self, box_pos: &Position, r#move: &Move) -> bool {
        let target_pos = Position::apply_move(box_pos, r#move);

        match &self.grid[target_pos.0][target_pos.1] {
            Cell::Wall => false,
            Cell::Empty => {
                self.grid[target_pos.0][target_pos.1] = Cell::Box;
                self.grid[box_pos.0][box_pos.1] = Cell::Empty;
                true
            }
            Cell::Box => {
                if self.try_move_box(&target_pos, r#move) {
                    self.grid[target_pos.0][target_pos.1] = Cell::Box;
                    self.grid[box_pos.0][box_pos.1] = Cell::Empty;
                    true
                } else {
                    false
                }
            }
            v => panic!("Unsupported CellKind for part 1 {:?}", v),
        }
    }

    pub fn apply_robot_moves(&mut self) {
        if self.debugger.is_activated {
            self.debugger.debug_initial_state(&self.to_string());
        }

        for r#move in self.robot.moves.clone().into_iter() {
            let target_cell_position = Position::apply_move(&self.robot.position, &r#move);
            let target_cell = &self.grid[target_cell_position.0][target_cell_position.1];

            match &target_cell {
                Cell::Wall => continue,
                Cell::Empty => {
                    self.robot.position = target_cell_position;
                }
                Cell::Box => {
                    if self.try_move_box(&target_cell_position, &r#move) {
                        self.robot.position = target_cell_position;
                    }
                }
                _ => {
                    if self.try_move_wide_box(&target_cell_position, &r#move, false) {
                        self.robot.position = target_cell_position;
                    }
                }
            }

            if self.debugger.is_activated {
                self.debugger.debug_move(&r#move, &self.to_string());
            }
        }
    }
}
