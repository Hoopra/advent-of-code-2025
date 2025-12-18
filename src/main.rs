mod day_01;
mod day_02;
mod day_03;
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
        3 => {
            println!("part 1: {}", day_03::solve_part_1());
            println!("part 2: {}", day_03::solve_part_2());
        }
        _ => {
            println!("not yet solved");
        }
    }
}
