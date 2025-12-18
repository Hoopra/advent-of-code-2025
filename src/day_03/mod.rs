mod battery;

use crate::util::read_input;
use battery::BatteryBank;

pub fn solve_part_1() -> u128 {
    let input = read_input("src/day_03/input.txt");

    let banks = BatteryBank::multiple_from_string(input);
    BatteryBank::best_joltages_two_batteries(&banks)
}

pub fn solve_part_2() -> u128 {
    let input = read_input("src/day_03/input.txt");

    let banks = BatteryBank::multiple_from_string(input);
    BatteryBank::best_joltages(&banks, 12)
}
