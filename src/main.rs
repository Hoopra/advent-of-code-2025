mod day_01;
mod day_02;
mod day_04;
mod day_05;
mod day_06;
mod util;

use std::io::stdin;

fn main() {
    println!("solve for day: ");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();

    let day = buffer.trim().parse().expect("type a valid number");

    println!("day {day}");
    match day {
        1 => {
            println!("part 1: {}", day_01::solve_part_1());
            println!("part 2: {}", day_01::solve_part_2());
        }
        2 => {
            println!("part 1: {}", day_02::solve_part_1());
            println!("part 2: {}", day_02::solve_part_2());
        }
        4 => {
            println!("part 1: {}", day_04::solve_part_1());
            println!("part 2: {}", day_04::solve_part_2());
        }
        5 => {
            println!("part 1: {}", day_05::solve_part_1());
            println!("part 2: {}", day_05::solve_part_2());
        }
        6 => {
            println!("part 1: {}", day_06::solve_part_1());
            println!("part 2: {}", day_06::solve_part_2());
        }
        _ => {
            println!("not yet solved");
        }
    }
}
