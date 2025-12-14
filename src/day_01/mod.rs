mod dial;
mod dial_traverse;
mod direction;

use crate::{day_01::dial_traverse::TraverseZeroDial, util::read_input};
use dial::Dial;

pub fn solve_part_1() -> u32 {
    let sequence = read_input("src/day_01/input.txt");

    let mut dial = Dial::new();
    dial.move_sequence(&sequence);

    dial.zeroes
}

pub fn solve_part_2() -> u32 {
    let sequence = read_input("src/day_01/input.txt");

    let mut dial = TraverseZeroDial::new();
    dial.move_sequence(&sequence);

    dial.zeroes
}
