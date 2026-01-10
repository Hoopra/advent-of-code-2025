use std::cmp::{max, min};

#[cfg(test)]
use super::coordinate::coordinates_from_string;
use super::coordinate_range::CoordinateRange;
use super::rectangle::Rectangle;
use crate::util::Position2D;

#[derive(Debug, PartialEq)]
pub struct TilePattern {
    enclosing_ranges: Vec<CoordinateRange>,
}

impl TilePattern {
    #[cfg(test)]
    pub fn from_string(input: &str) -> Self {
        let coordinates = coordinates_from_string(input);
        Self::from_coordinates(&coordinates)
    }

    pub fn from_coordinates(coordinates: &[Position2D]) -> Self {
        let inner_ranges: Vec<CoordinateRange> = coordinates
            .iter()
            .enumerate()
            .skip(1)
            .map(|(index, coordinate)| {
                let previous = coordinates.get(index - 1).unwrap();

                let (xp, yp) = *previous;
                let (xc, yc) = *coordinate;

                match xp == xc {
                    true => {
                        let y_min = min(yp, yc);
                        let y_max = max(yp, yc);

                        CoordinateRange::Y(xp, (y_min, y_max))
                    }
                    false => {
                        let x_min = min(xp, xc);
                        let x_max = max(xp, xc);

                        CoordinateRange::X((x_min, x_max), yp)
                    }
                }
            })
            .collect();

        let enclosing_ranges = inner_ranges
            .iter()
            .enumerate()
            .map(|(index, range)| {
                let previous = inner_ranges.get(index - 1);
                let next = inner_ranges.get(index + 1);

                let new_range = match range {
                    CoordinateRange::X((x_min, x_max), y) => {
                        let range = CoordinateRange::X((x_min - 1, x_max + 1), *y);

                        range
                    }
                    CoordinateRange::Y(x, (y_min, y_max)) => {
                        

                        let range = CoordinateRange::Y(*x, (y_min - 1, y_max + 1));

                        range
                    }
                };

                new_range
            })
            .collect();

        Self { enclosing_ranges }
    }
}

impl TilePattern {
    pub fn includes_rectangle(&self, rectangle: &Rectangle) -> bool {
        for rectangle_range in &rectangle.ranges {
            for range in &self.enclosing_ranges {
                if range.touches(rectangle_range) {
                    dbg!(range, rectangle_range);
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
    fn pattern_includes_rectangle() {
        let corner_1 = (9, 5);
        let corner_2 = (2, 3);

        // ..............
        // .......#...#..
        // ..............
        // ..#AAAA#AA....
        // ..A......A....
        // ..#AAAAAA#....
        // ..............
        // .........#.#..
        // ..............

        // ..............
        // ..............
        // ..............
        // ...b..........
        // ...b..........
        // ..axaaaaaa....
        // ..............
        // ..............
        // ..............

        let input = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3";
        let pattern = TilePattern::from_string(input);
        let rectangle = Rectangle::from_corners(&corner_1, &corner_2);

        dbg!(&rectangle.ranges);

        assert_eq!(pattern.includes_rectangle(&rectangle), true);
    }
}
