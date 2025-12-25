mod splitter;

use crate::util::read_input;
use splitter::Map2D;

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_07/input.txt");

    Map2D::from_string(&input).count_splits()
}

pub fn solve_part_2() -> usize {
    let input = read_input("src/day_07/input.txt");

    0
}
