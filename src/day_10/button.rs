use super::indicator::{State, reset_indicators};
use std::collections::HashMap;

pub type Button = Vec<usize>;

pub fn buttons_from_string(characters: Vec<&str>) -> Vec<Button> {
    characters
        .into_iter()
        .take_while(|entry| !entry.contains("{"))
        .map(button_from_string)
        .collect()
}

pub fn button_from_string(input: &str) -> Button {
    input
        .chars()
        .filter_map(|character| character.to_digit(10).map(|entry| entry as usize))
        .collect()
}

#[derive(Debug, PartialEq, Eq)]

pub struct ButtonCombination {
    pub buttons: Vec<Button>,
    pub discrepancy: usize,
}

impl ButtonCombination {
    pub fn new(button: &Button, target_indicators: &HashMap<usize, State>) -> Self {
        let mut result = Self {
            buttons: vec![button.clone()],
            discrepancy: 100,
        };

        result.discrepancy = result.calculate_discrepancy(target_indicators);
        result
    }

    pub fn with_buttons(buttons: Vec<Button>, target_indicators: &HashMap<usize, State>) -> Self {
        let mut result = Self {
            buttons,
            discrepancy: 100,
        };

        result.discrepancy = result.calculate_discrepancy(target_indicators);
        result
    }
}

impl ButtonCombination {
    fn apply(&self, target_indicators: &HashMap<usize, State>) -> HashMap<usize, State> {
        let ButtonCombination { buttons, .. } = self;

        let mut indicators = reset_indicators(target_indicators);

        for button in buttons {
            for index in button {
                let current = indicators.get(index).unwrap();
                indicators.insert(
                    *index,
                    match current {
                        State::On => State::Off,
                        _ => State::On,
                    },
                );
            }
        }

        indicators
    }

    fn calculate_discrepancy(&self, target_indicators: &HashMap<usize, State>) -> usize {
        let indicators = self.apply(target_indicators);

        indicators
            .into_iter()
            .filter(|(index, state)| {
                let target = target_indicators.get(index);
                let current = Some(state);

                current != target
            })
            .count()
    }

    pub fn score(&self) -> usize {
        // 2 * self.buttons.len() + self.discrepancy
        self.buttons.len()
    }

    pub fn add_best_button(
        &self,
        all_buttons: &Vec<Button>,
        target_indicators: &HashMap<usize, State>,
    ) -> Vec<Self> {
        let ButtonCombination { buttons, .. } = self;

        let possible_buttons: Vec<&Vec<usize>> = all_buttons
            .iter()
            .filter(|button| !buttons.contains(button))
            .collect();

        let mut result: Vec<ButtonCombination> = possible_buttons
            .into_iter()
            .map(|button| {
                let mut combination_buttons = buttons.clone();
                combination_buttons.push(button.clone());

                ButtonCombination::with_buttons(combination_buttons, target_indicators)
            })
            .collect();

        result.sort();

        result
    }
}

impl PartialOrd for ButtonCombination {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.score().partial_cmp(&self.score())
    }
}

impl Ord for ButtonCombination {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score().cmp(&self.score())
    }
}
