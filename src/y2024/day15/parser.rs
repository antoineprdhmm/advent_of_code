use crate::read_input;

use super::{
    debugger::WarehouseDebugger,
    warehouse::{Cell, Move, Position, Robot, Warehouse},
};

fn parse_cells(line: &str) -> Vec<Cell> {
    line.chars()
        .map(|c| match c {
            '#' => Cell::Wall,
            'O' => Cell::Box,
            _ => Cell::Empty,
        })
        .collect::<Vec<_>>()
}

fn parse_moves(line: &str) -> Vec<Move> {
    line.chars()
        .map(|c| match c {
            '<' => Move::LEFT,
            '>' => Move::RIGHT,
            '^' => Move::UP,
            'v' => Move::DOWN,
            invalid => panic!("Invalid direction found while parsing input: {}", invalid),
        })
        .collect::<Vec<_>>()
}

pub fn init_warehouse() -> Warehouse {
    let mut empty_line_found = false;
    let lines = read_input(2024, 15).unwrap().map_while(Result::ok);

    let mut grid: Vec<Vec<Cell>> = vec![];
    let mut moves: Vec<Move> = vec![];
    let mut robot_position = Position(0, 0);

    for (row, line) in lines.enumerate() {
        if line.is_empty() {
            empty_line_found = true;
            continue;
        }

        if empty_line_found {
            moves.extend(parse_moves(&line));
        } else {
            let robot_index = line.find("@");
            if let Some(col) = robot_index {
                robot_position = Position(row, col);
            }

            grid.push(parse_cells(&line));
        }
    }

    Warehouse {
        grid,
        robot: Robot {
            position: robot_position,
            moves,
        },
        debugger: WarehouseDebugger::default(),
    }
}
