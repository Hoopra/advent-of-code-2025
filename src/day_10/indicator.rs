use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    On,
    Off,
}

pub fn reset_indicators(indicators: &HashMap<usize, State>) -> HashMap<usize, State> {
    indicators
        .clone()
        .into_iter()
        .map(|(index, _)| (index, State::Off))
        .collect()
}

pub fn indicators_from_string(input: &str) -> HashMap<usize, State> {
    let mut result = HashMap::new();
    let mut index = 0;

    input.chars().for_each(|character| match character {
        '.' => {
            result.insert(index, State::Off);
            index += 1;
        }
        '#' => {
            result.insert(index, State::On);
            index += 1;
        }
        _ => {}
    });

    result
}
