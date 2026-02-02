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

            machine.least_button_presses_for_target().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determines_least_button_presses() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

        assert_eq!(least_button_presses_for_machines(input), 7);
    }
}
