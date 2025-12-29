mod circuit;

use crate::util::read_input;
use circuit::{Junction, connect_junctions, find_global_connection_point, flatten_circuits};

pub fn solve_part_1() -> u64 {
    let input = read_input("src/day_08/input.txt");

    multiply_largest_circuits(&input, 3, 1000)
}

pub fn solve_part_2() -> u64 {
    let input = read_input("src/day_08/input.txt");

    let junctions = Junction::many_from_string(&input);
    let (junction_1, junction_2) = find_global_connection_point(&junctions).unwrap();

    junction_1.x * junction_2.x
}

fn multiply_largest_circuits(input: &str, number: usize, connections: usize) -> u64 {
    let junctions = Junction::many_from_string(input);
    let circuits = connect_junctions(&junctions, connections);

    let circuit_lengths = flatten_circuits(&circuits);

    circuit_lengths
        .iter()
        .take(number)
        .fold(1, |result, next| result * next)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplies_largest_circuit_groups() {
        let input = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689";

        let result = multiply_largest_circuits(input, 3, 10);
        assert_eq!(result, 40)
    }
}
