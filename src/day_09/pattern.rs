#[cfg(test)]
use super::coordinate::coordinates_from_string;
use super::coordinate_range::{CoordinateRange, Direction};
use super::rectangle::Rectangle;
use crate::util::Position2D;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct TilePattern {
    perimeter: Vec<CoordinateRange>,
}

impl TilePattern {
    #[cfg(test)]
    pub fn from_string(input: &str) -> Self {
        let coordinates = coordinates_from_string(input);
        Self::from_coordinates(&coordinates)
    }

    fn construct_out_directions(
        inner_ranges: &Vec<CoordinateRange>,
    ) -> HashMap<&CoordinateRange, Direction> {
        let mut out_directions: HashMap<&CoordinateRange, Direction> = HashMap::new();

        let (mut index, top_range) = CoordinateRange::find_top(&inner_ranges);

        out_directions.insert(top_range, Direction::Up);

        while out_directions.len() != inner_ranges.len() {
            let previous = inner_ranges.get(index).unwrap();
            let previous_out_direction = out_directions.get(previous).unwrap();

            if index == 363 {}

            index = (index + 1) % inner_ranges.len();
            let next = inner_ranges.get(index).unwrap();
            let current_direction = next.direction();

            if index == 364 {}

            let out_direction = match previous_out_direction {
                Direction::Down => match current_direction {
                    Direction::Up => Direction::Left,
                    _ => Direction::Right,
                },
                Direction::Up => match current_direction {
                    Direction::Up => Direction::Left,
                    _ => Direction::Right,
                },
                Direction::Left => match current_direction {
                    Direction::Right => Direction::Up,
                    _ => Direction::Down,
                },
                Direction::Right => match current_direction {
                    Direction::Right => Direction::Up,
                    _ => Direction::Down,
                },
            };

            out_directions.insert(next, out_direction);
        }

        out_directions
    }

    fn construct_perimeter(
        inner_ranges: &Vec<CoordinateRange>,
        out_directions: &HashMap<&CoordinateRange, Direction>,
    ) -> Vec<CoordinateRange> {
        inner_ranges
            .iter()
            .enumerate()
            .map(|(index, range)| {
                let previous_index = match index > 0 {
                    true => index - 1,
                    _ => inner_ranges.len() - 1,
                };

                let next_index = match index < inner_ranges.len() - 1 {
                    true => index + 1,
                    _ => 0,
                };

                let previous = inner_ranges.get(previous_index).unwrap();
                let next = inner_ranges.get(next_index).unwrap();

                let (start, end) = range.get_points();
                let (mut x_start, mut y_start) = start.clone();
                let (mut x_end, mut y_end) = end.clone();

                let out_direction = out_directions.get(range).unwrap();

                match out_direction {
                    Direction::Down => {
                        y_end += 1;
                        y_start += 1;
                    }
                    Direction::Up => {
                        y_end -= 1;
                        y_start -= 1;
                    }
                    Direction::Left => {
                        x_end -= 1;
                        x_start -= 1;
                    }
                    Direction::Right => {
                        x_end += 1;
                        x_start += 1;
                    }
                }

                let start = (x_start, y_start);
                let end = (x_end, y_end);

                let (x_center, y_center) = range.find_center();

                if next.contains_point(&start) || previous.contains_point(&start) {
                    match out_direction {
                        Direction::Left | Direction::Right => match y_start > y_center {
                            true => y_start -= 1,
                            _ => y_start += 1,
                        },

                        Direction::Up | Direction::Down => match x_start > x_center {
                            true => x_start -= 1,
                            _ => x_start += 1,
                        },
                    }
                }

                if next.contains_point(&end) || previous.contains_point(&end) {
                    match out_direction {
                        Direction::Left | Direction::Right => match y_end > y_center {
                            true => y_end += 1,
                            _ => y_end -= 1,
                        },

                        Direction::Up | Direction::Down => match x_end > x_center {
                            true => x_end -= 1,
                            _ => x_end += 1,
                        },
                    }
                }

                CoordinateRange::new((x_start, y_start), (x_end, y_end))
            })
            .collect()
    }

    pub fn from_coordinates(coordinates: &[Position2D]) -> Self {
        let inner_ranges = CoordinateRange::many_from_coordinates(coordinates);
        let out_directions = Self::construct_out_directions(&inner_ranges);
        let perimeter = Self::construct_perimeter(&inner_ranges, &out_directions);

        Self { perimeter }
    }
}

impl TilePattern {
    pub fn includes_rectangle(&self, rectangle: &Rectangle) -> bool {
        for rectangle_range in &rectangle.ranges {
            for range in &self.perimeter {
                if range.intersects(rectangle_range) {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determines_outward_directions() {
        let coordinates = coordinates_from_string("1,1\n5,1\n5,5\n1,5");
        let ranges = CoordinateRange::many_from_coordinates(&coordinates);
        let directions = TilePattern::construct_out_directions(&ranges);

        assert_eq!(
            directions,
            HashMap::from([
                (&CoordinateRange::new((1, 1), (5, 1)), Direction::Up),
                (&CoordinateRange::new((5, 1), (5, 5)), Direction::Right),
                (&CoordinateRange::new((5, 5), (1, 5)), Direction::Down),
                (&CoordinateRange::new((1, 5), (1, 1)), Direction::Left),
            ])
        );

        let coordinates = coordinates_from_string("7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3");
        let ranges = CoordinateRange::many_from_coordinates(&coordinates);
        let directions = TilePattern::construct_out_directions(&ranges);

        assert_eq!(
            directions,
            HashMap::from([
                (&CoordinateRange::new((7, 1), (11, 1)), Direction::Up),
                (&CoordinateRange::new((11, 1), (11, 7)), Direction::Right),
                (&CoordinateRange::new((11, 7), (9, 7)), Direction::Down),
                (&CoordinateRange::new((9, 7), (9, 5)), Direction::Left),
                (&CoordinateRange::new((9, 5), (2, 5)), Direction::Down),
                (&CoordinateRange::new((2, 5), (2, 3)), Direction::Left),
                (&CoordinateRange::new((2, 3), (7, 3)), Direction::Up),
                (&CoordinateRange::new((7, 3), (7, 1)), Direction::Left),
            ])
        );
    }

    #[test]
    fn constructs_perimeter() {
        let coordinates = coordinates_from_string("1,1\n5,1\n5,5\n1,5");
        let pattern = TilePattern::from_coordinates(&coordinates);

        // .O...O.
        // O#...#O
        // .......
        // .......
        // .......
        // O#...#.
        // .O...O.

        assert_eq!(
            pattern.perimeter,
            vec![
                CoordinateRange::new((1, 0), (5, 0)),
                CoordinateRange::new((6, 1), (6, 5)),
                CoordinateRange::new((5, 6), (1, 6)),
                CoordinateRange::new((0, 5), (0, 1)),
            ]
        );

        // .......O...O..
        // ......O#...#O.
        // ..O...O.......
        // .O#....#......
        // ..............
        // .O#......#....
        // ..O.....O.....
        // ........O#.#O.
        // .........O.O..

        let input = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3";
        let pattern = TilePattern::from_string(input);

        assert_eq!(
            pattern.perimeter,
            vec![
                CoordinateRange::new((7, 0), (11, 0)),
                CoordinateRange::new((12, 1), (12, 7)),
                CoordinateRange::new((11, 8), (9, 8)),
                CoordinateRange::new((8, 7), (8, 6)),
                CoordinateRange::new((8, 6), (2, 6)),
                CoordinateRange::new((1, 5), (1, 3)),
                CoordinateRange::new((2, 2), (6, 2)),
                CoordinateRange::new((6, 2), (6, 1)),
            ]
        );
    }

    #[test]
    fn pattern_includes_rectangle() {
        let input = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3";
        let pattern = TilePattern::from_string(input);
        let rectangle = Rectangle::from_corners(&(9, 5), &(2, 3));

        // ..............
        // .......O...#..
        // ..............
        // ..#AAAA#AA....
        // ..A......A....
        // ..#AAAAAA#....
        // ..............
        // .........#.O..
        // ..............

        assert_eq!(pattern.includes_rectangle(&rectangle), true);

        let rectangle = Rectangle::from_corners(&(7, 1), &(11, 7));

        // ..............
        // .......#AAA#..
        // .......A...A..
        // ..#....#...A..
        // .......A...A..
        // ..#....A.#.A..
        // .......A...A..
        // .......AA#A#..
        // ..............

        assert_eq!(pattern.includes_rectangle(&rectangle), false);
    }
}
