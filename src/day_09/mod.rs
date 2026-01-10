mod coordinate;
mod coordinate_range;
mod pattern;
mod rectangle;

use std::cmp::max;

use crate::util::{Position2D, read_input};
use coordinate::coordinates_from_string;
use pattern::TilePattern;
use rectangle::Rectangle;

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_09/input.txt");

    let coordinates = coordinates_from_string(&input);
    largest_rectangle(&coordinates)
}

pub fn solve_part_2() -> usize {
    let input = read_input("src/day_09/input.txt");

    let coordinates = coordinates_from_string(&input);
    largest_rectangle_colored_tiles(&coordinates)
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

            let area = Rectangle::from_corners(first, second).area;

            largest = max(largest, area);
        }
    }

    largest
}

pub fn largest_rectangle_colored_tiles(coordinates: &[Position2D]) -> usize {
    let pattern = TilePattern::from_coordinates(coordinates);

    let mut largest = 0;

    for first in coordinates.iter() {
        let (x1, y1) = first;

        for second in coordinates.iter() {
            let (x2, y2) = second;
            let rectangle = Rectangle::from_corners(first, second);

            if x1 == x2 && y1 == y2 || !pattern.includes_rectangle(&rectangle) {
                continue;
            }

            let area = Rectangle::from_corners(first, second).area;

            largest = max(largest, area);
        }
    }

    largest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_largest_possible_rectangle() {
        let input = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3";

        let coordinates = coordinates_from_string(input);
        let result = largest_rectangle(&coordinates);

        assert_eq!(result, 50);
    }

    #[test]
    fn finds_largest_possible_rectangle_all_colors() {
        let input = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3";

        let coordinates = coordinates_from_string(input);
        let result = largest_rectangle_colored_tiles(&coordinates);

        assert_eq!(result, 24);
    }
}
