use super::coordinate_range::CoordinateRange;
use super::util::find_min_max;

pub struct Rectangle {
    pub area: usize,
    pub ranges: Vec<CoordinateRange>,
}

impl Rectangle {
    pub fn from_corners(start: &(usize, usize), end: &(usize, usize)) -> Self {
        let (x_start, y_start) = *start;
        let (x_end, y_end) = *end;

        let (x_min, x_max) = find_min_max(x_start, x_end);
        let (y_min, y_max) = find_min_max(y_start, y_end);

        let side_1 = y_max - y_min + 1;
        let side_2 = x_max - x_min + 1;

        let area = side_1 * side_2;

        Self {
            area,
            ranges: vec![
                CoordinateRange::new((x_min, y_min), (x_min, y_max)),
                CoordinateRange::new((x_max, y_min), (x_max, y_max)),
                CoordinateRange::new((x_min, y_min), (x_max, y_min)),
                CoordinateRange::new((x_max, y_max), (x_max, y_max)),
            ],
        }
    }
}
