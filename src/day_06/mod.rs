mod assignment;

use crate::util::read_input;
use assignment::{assignment_results_from_string, cephalopod_assignment_results_from_string};

pub fn solve_part_1() -> u64 {
    let input = read_input("src/day_06/input.txt");

    assignment_results_from_string(&input)
}

pub fn solve_part_2() -> u64 {
    let input = read_input("src/day_06/input.txt");

    cephalopod_assignment_results_from_string(&input)
}
