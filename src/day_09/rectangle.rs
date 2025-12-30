use crate::util::Position2D;
use std::cmp::{max, min};

fn to_coordinate(entry: Option<&str>) -> usize {
    entry.unwrap().parse().unwrap()
}

pub fn coordinates_from_string(input: &str) -> Vec<Position2D> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            (to_coordinate(parts.next()), to_coordinate(parts.next()))
        })
        .collect()
}

fn rectangle_area(corner_1: &Position2D, corner_2: &Position2D) -> usize {
    let (x1, y1) = corner_1;
    let (x2, y2) = corner_2;

    let side_1 = max(x1, x2) - min(x1, x2) + 1;
    let side_2 = max(y1, y2) - min(y1, y2) + 1;

    side_1 * side_2
}

pub fn largest_rectangle(coordinates: &[Position2D]) -> usize {
    let mut largest = 0;

    for first in coordinates.iter() {
        let (x1, y1) = first;

        for second in coordinates.iter() {
            let (x2, y2) = second;

            if x1 == x2 && y1 == y2 {
                continue;
            }

            let area = rectangle_area(first, second);

            largest = max(largest, area);
        }
    }

    largest
}

#[cfg(test)]
mod tests {
    use crate::day_09::rectangle::{coordinates_from_string, largest_rectangle};

    #[test]
    fn finds_largest_possible_rectangle() {
        let input = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3";

        let coordinates = coordinates_from_string(input);
        let result = largest_rectangle(&coordinates);

        assert_eq!(result, 50);
    }
}
