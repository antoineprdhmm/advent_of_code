mod debugger;
mod parser;
mod warehouse;

use parser::init_warehouse;

pub fn run_part_1() {
    let mut warehouse = init_warehouse();
    warehouse.apply_robot_moves();
    println!("{}", warehouse.compute_gps_coordinates_sum());
}

pub fn run_part_2() {
    let mut warehouse = init_warehouse();
    warehouse.make_wide();
    warehouse.debugger.activate_debug();
    warehouse.apply_robot_moves();
    println!("{}", warehouse.compute_gps_coordinates_sum());
}
