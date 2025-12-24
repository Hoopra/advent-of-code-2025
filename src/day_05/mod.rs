mod inventory;

use crate::{day_05::inventory::inventory_from_string, util::read_input};
use inventory::fresh_parts_from_string;

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_05/input.txt");

    fresh_parts_from_string(&input)
}

pub fn solve_part_2() -> usize {
    let input = read_input("src/day_05/input.txt");

    let inventory = inventory_from_string(&input);
    inventory.total_indices()
}
