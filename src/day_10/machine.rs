use std::collections::{BinaryHeap, HashMap};

use super::{
    button::{Button, ButtonCombination, buttons_from_string},
    indicator::{State, indicators_from_string},
};

#[derive(Debug)]
pub struct Machine {
    target_indicators: HashMap<usize, State>,
    buttons: Vec<Button>,
}

impl Machine {
    pub fn from_string(input: &str) -> Self {
        let mut parts = input.split(' ');

        let target_indicators = indicators_from_string(parts.next().unwrap());
        let buttons = buttons_from_string(parts.collect());

        Machine {
            target_indicators,
            buttons,
        }
    }
}

impl Machine {
    pub fn least_button_presses_for_target(self) -> Option<usize> {
        let Self {
            target_indicators,
            buttons: all_buttons,
        } = self;

        let initial_combinations = all_buttons
            .iter()
            .map(|button| ButtonCombination::new(button, &target_indicators))
            .collect::<Vec<ButtonCombination>>();

        let mut queue: BinaryHeap<ButtonCombination> = BinaryHeap::from(initial_combinations);

        while queue.len() > 0 {
            let next = queue.pop().unwrap();

            if next.discrepancy == 0 {
                return Some(next.buttons.len());
            }

            let new_combinations = next.add_best_button(&all_buttons, &target_indicators);

            for combination in new_combinations {
                queue.push(combination);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_machine_from_string() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machine = Machine::from_string(input);

        assert_eq!(
            machine.target_indicators,
            HashMap::from([
                (0, State::Off),
                (1, State::On),
                (2, State::On),
                (3, State::Off),
            ])
        );

        assert_eq!(
            machine.buttons,
            vec![
                vec![3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![0, 2],
                vec![0, 1]
            ]
        );
    }

    #[test]
    fn determines_least_button_presses() {
        let machine = Machine::from_string("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        assert_eq!(machine.least_button_presses_for_target(), Some(2));

        let machine =
            Machine::from_string("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        assert_eq!(machine.least_button_presses_for_target(), Some(3));

        let machine = Machine::from_string(
            "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        );
        assert_eq!(machine.least_button_presses_for_target(), Some(2));

        let machine = Machine::from_string("[#.#.] (0,1,2,3) (0,2) {25,9,25,9}");
        assert_eq!(machine.least_button_presses_for_target(), Some(1));

        let machine = Machine::from_string("[#.#.] (0,2) (0,1,2,3) {25,9,25,9}");
        assert_eq!(machine.least_button_presses_for_target(), Some(1));

        let machine = Machine::from_string("[..##] (1,3) (0,2) (1,2,3) (0,3) (3) {22,169,31,198}");
        assert_eq!(machine.least_button_presses_for_target(), Some(2));

        let machine = Machine::from_string(
            "[######..] (1,2,3,5) (1,2,3,6,7) (2,4,5,7) (2,3,4,5,6) (0,1,2,7) (0,1,2,4,5,6,7) {17,46,55,34,19,28,35,41}",
        );
        assert_eq!(machine.least_button_presses_for_target(), Some(3));
    }

    #[test]
    fn determines_discrepancy() {
        let input = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let machine = Machine::from_string(input);

        let buttons = buttons_from_string(vec!["(0,4)", "(0,1,2)", "(1,2,3,4)"]);
        let combination = ButtonCombination::with_buttons(buttons, &machine.target_indicators);

        assert_eq!(combination.discrepancy, 0);
    }
}
