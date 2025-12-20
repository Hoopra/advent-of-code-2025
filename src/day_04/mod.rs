mod map;

use crate::util::read_input;
use map::Map;

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_04/input.txt");

    let map = Map::from_string(&input);
    map.rolls_with_max_neighbors(4)
}

pub fn solve_part_2() -> usize {
    let input = read_input("src/day_04/input.txt");

    let mut map = Map::from_string(&input);
    map.remove_all_accessible_rolls(4)
}
