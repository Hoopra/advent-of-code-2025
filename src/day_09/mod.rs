mod rectangle;

use crate::util::read_input;
use rectangle::{coordinates_from_string, largest_rectangle};

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_09/input.txt");

    let coordinates = coordinates_from_string(&input);
    largest_rectangle(&coordinates)
}

pub fn solve_part_2() -> u64 {
    let input = read_input("src/day_09/input.txt");

    0
}
