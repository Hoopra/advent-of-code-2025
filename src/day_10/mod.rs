mod button;
mod indicator;
mod machine;

use crate::util::read_input;
use machine::Machine;

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_10/input.txt");

    least_button_presses_for_machines(&input)
}

pub fn solve_part_2() -> usize {
    let _input = read_input("src/day_10/input.txt");

    0
}

fn least_button_presses_for_machines(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let machine = Machine::from_string(line);
            dbg!(line);
            machine.least_button_presses_for_target().unwrap()
        })
        .sum()
}
