use keypad_layering_system::KeypadLayeringSystem;

mod dijkstra;
mod keypad;
mod keypad_layering_system;

use crate::read_input;

fn run(nb_layers: usize) -> usize {
    let mut kls = KeypadLayeringSystem::new(nb_layers);

    read_input(2024, 21)
        .unwrap()
        .map_while(Result::ok)
        .map(|code| {
            let s_len = kls.get_fewest_number_button_press_for_code(&code);
            let digit = code[0..3].parse::<usize>().unwrap_or(0);

            s_len * digit
        })
        .sum()
}

pub fn run_part_1() {
    let result = run(2);
    assert_eq!(result, 278748);
}

pub fn run_part_2() {
    let result = run(25);
    assert_eq!(result, 337744744231414);
}
