use std::collections::HashMap;

use super::keypad::{Keypad, DIRECTIONAL_KEYPAD, NUMERIC_KEYPAD};

pub struct KeypadLayeringSystem {
    numerical_keypad: Keypad,
    directional_keypad: Keypad,
    nb_directional_layers: usize,
    cache: HashMap<(char, char, usize), usize>,
}

impl KeypadLayeringSystem {
    pub fn new(nb_directional_layers: usize) -> KeypadLayeringSystem {
        KeypadLayeringSystem {
            numerical_keypad: Keypad::new(&NUMERIC_KEYPAD),
            directional_keypad: Keypad::new(&DIRECTIONAL_KEYPAD),
            nb_directional_layers,
            cache: HashMap::new(),
        }
    }

    fn get_shortest_sequence_length_between_keys(&mut self, from: &char, to: &char) -> usize {
        let shortest_numerical_keypad_sequences: Vec<String> = self
            .numerical_keypad
            .find_keypad_shortest_sequences(from.to_string().as_str(), to.to_string().as_str());

        let mut shortest_sequence_length = usize::MAX;

        for sequence in shortest_numerical_keypad_sequences {
            // Put a A in front, because we start from A key
            let sequence = "A".to_string() + &sequence;

            // Check the length of each sequence at the upper directional layer
            let length = sequence
                .chars()
                .collect::<Vec<_>>()
                .windows(2)
                .map(|pair| {
                    self.apply_directional_layers(
                        &pair[0],
                        &pair[1],
                        self.nb_directional_layers - 1,
                    )
                })
                .sum();

            // Keep track of the shortest sequence length
            shortest_sequence_length = shortest_sequence_length.min(length);
        }

        shortest_sequence_length
    }

    // Recursive approach to apply the directional layers
    // Return the shortest sequence length between 2 keys
    // Uses memoization
    fn apply_directional_layers(
        &mut self,
        from: &char,
        to: &char,
        remaining_nb_layers: usize,
    ) -> usize {
        if let Some(cached_value) = self.cache.get(&(*from, *to, remaining_nb_layers)) {
            return *cached_value;
        }

        let shortest_directional_sequences: Vec<String> = self
            .directional_keypad
            .find_keypad_shortest_sequences(from.to_string().as_str(), to.to_string().as_str());

        if remaining_nb_layers == 0 {
            // We reached the upper layer
            // As it's the last layer, we know for sure these sequences are the shortest one for these layers
            // Because we don't build other sequences on top of them
            // -> We build up the response from there
            return shortest_directional_sequences
                .iter()
                .min_by_key(|s| s.len())
                .map_or(0, |s| s.len());
        }

        let mut shortest_sequence_length = usize::MAX;

        for sequence in shortest_directional_sequences {
            // Put a A in front, because we start from A key
            let sequence = "A".to_string() + &sequence;

            let length = sequence
                .chars()
                .collect::<Vec<_>>()
                .windows(2)
                .map(|pair| {
                    self.apply_directional_layers(&pair[0], &pair[1], remaining_nb_layers - 1)
                })
                .sum();

            shortest_sequence_length = shortest_sequence_length.min(length);
        }

        self.cache
            .insert((*from, *to, remaining_nb_layers), shortest_sequence_length);

        shortest_sequence_length
    }

    pub fn get_fewest_number_button_press_for_code(&mut self, code: &str) -> usize {
        // Put a A in front, because we start from A key on the numerical keypad
        let keys = ("A".to_string() + code).chars().collect::<Vec<_>>();

        keys.windows(2)
            .map(|pair| self.get_shortest_sequence_length_between_keys(&pair[0], &pair[1]))
            .sum()
    }
}
