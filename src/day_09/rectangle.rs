use super::coordinate_range::CoordinateRange;
use std::cmp::{max, min};

pub struct Rectangle {
    pub area: usize,
    pub ranges: Vec<CoordinateRange>,
}

impl Rectangle {
    pub fn from_corners(first: &(usize, usize), second: &(usize, usize)) -> Self {
        let (x1, y1) = *first;
        let (x2, y2) = *second;

        let x_min = min(x1, x2);
        let x_max = max(x1, x2);

        let y_min = min(y1, y2);
        let y_max = max(y1, y2);

        let side_1 = y_max - y_min + 1;
        let side_2 = x_max - x_min + 1;

        let area = side_1 * side_2;

        Self {
            area,
            ranges: vec![
                CoordinateRange::X((x_min, x_max), y_min),
                CoordinateRange::Y(x_min, (y_min, y_max)),
                CoordinateRange::X((x_min, x_max), y_max),
                CoordinateRange::Y(y_min, (y_min, y_max)),
            ],
        }
    }
}
