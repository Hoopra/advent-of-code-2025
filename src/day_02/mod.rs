mod range;

use crate::util::read_input;
use range::Range;

pub fn solve_part_1() -> u64 {
    let input = read_input("src/day_02/input.txt");

    let ranges = Range::many_from_string(&input);
    Range::sum_duplicate_once_ids_in_ranges(&ranges)
}

pub fn solve_part_2() -> u64 {
    let input = read_input("src/day_02/input.txt");

    let ranges = Range::many_from_string(&input);
    Range::sum_duplicate_at_least_once_ids_in_ranges(&ranges)
}
