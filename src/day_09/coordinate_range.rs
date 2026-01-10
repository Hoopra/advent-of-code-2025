use super::util::find_min_max;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Hash)]
pub struct CoordinateRange {
    pub start: (usize, usize),
    pub end: (usize, usize),
}

impl CoordinateRange {
    pub fn new(start: (usize, usize), end: (usize, usize)) -> Self {
        Self { start, end }
    }

    pub fn many_from_coordinates(coordinates: &[(usize, usize)]) -> Vec<Self> {
        let mut ranges: Vec<CoordinateRange> = coordinates
            .iter()
            .enumerate()
            .skip(1)
            .map(|(index, coordinate)| {
                let previous = coordinates.get(index - 1).unwrap();
                CoordinateRange::new(*previous, *coordinate)
            })
            .collect();

        ranges.push(CoordinateRange::new(
            *coordinates.last().unwrap(),
            *coordinates.first().unwrap(),
        ));

        ranges
    }
}

impl CoordinateRange {
    fn overlaps_horizontally(&self, other: &CoordinateRange) -> bool {
        let (start_self, end_self) = self.get_points();
        let (start_other, end_other) = other.get_points();

        let (x_start_self, _) = *start_self;
        let (x_end_self, _) = *end_self;
        let (x_start_other, _) = *start_other;
        let (x_end_other, _) = *end_other;

        let (x_min, x_max) = find_min_max(x_start_other, x_end_other);

        let start_overlaps = x_start_self >= x_min && x_start_self <= x_max;
        let end_overlaps = x_end_self >= x_min && x_end_self <= x_max;

        start_overlaps || end_overlaps
    }

    fn overlaps_vertically(&self, other: &CoordinateRange) -> bool {
        let (start_self, end_self) = self.get_points();
        let (start_other, end_other) = other.get_points();

        let (_, y_start_self) = *start_self;
        let (_, y_end_self) = *end_self;
        let (_, y_start_other) = *start_other;
        let (_, y_end_other) = *end_other;

        let (y_min, y_max) = find_min_max(y_start_other, y_end_other);

        let start_overlaps = y_start_self >= y_min && y_start_self <= y_max;
        let end_overlaps = y_end_self >= y_min && y_end_self <= y_max;

        start_overlaps || end_overlaps
    }

    pub fn intersects(&self, other: &CoordinateRange) -> bool {
        (self.overlaps_horizontally(other) && other.overlaps_vertically(self))
            || (other.overlaps_horizontally(self) && self.overlaps_vertically(other))
    }

    pub fn contains_point(&self, other: &(usize, usize)) -> bool {
        let (start, end) = self.get_points();

        let (x, y) = *other;
        let (x_start, y_start) = *start;
        let (x_end, y_end) = *end;

        let (x_min, x_max) = find_min_max(x_start, x_end);
        let (y_min, y_max) = find_min_max(y_start, y_end);

        let contained_x = x >= x_min && x <= x_max;
        let contained_y = y >= y_min && y <= y_max;

        contained_x && contained_y
    }

    pub fn get_points(&self) -> (&(usize, usize), &(usize, usize)) {
        let CoordinateRange { start, end } = self;

        (start, end)
    }

    pub fn find_center(&self) -> (usize, usize) {
        let (start, end) = self.get_points();

        let (x_start, y_start) = *start;
        let (x_end, y_end) = *end;

        let (x_min, x_max) = find_min_max(x_start, x_end);
        let (y_min, y_max) = find_min_max(y_start, y_end);

        ((x_max - x_min) / 2, (y_max - y_min) / 2)
    }

    pub fn direction(&self) -> Direction {
        let CoordinateRange { start, end } = self;

        let (x_start, y_start) = start;
        let (x_end, y_end) = end;

        match x_start == x_end {
            true => match y_start > y_end {
                true => Direction::Up,
                _ => Direction::Down,
            },
            _ => match x_start > x_end {
                true => Direction::Left,
                _ => Direction::Right,
            },
        }
    }

    pub fn find_top(ranges: &[CoordinateRange]) -> (usize, &Self) {
        let first_range = ranges.first().unwrap();

        ranges
            .iter()
            .skip(1)
            .enumerate()
            .fold((0, first_range), |current, (index, next_range)| {
                let (_, current_range) = current;

                let CoordinateRange { start, end } = current_range;
                let (_, y_start_current) = start;
                let (_, y_end_current) = end;

                if y_start_current != y_end_current {
                    return (index + 1, next_range);
                }

                let CoordinateRange { start, end } = next_range;
                let (_, y_start) = start;
                let (_, y_end) = end;

                if y_start != y_end {
                    return current;
                }

                match y_start > y_start_current {
                    true => current,
                    _ => (index + 1, next_range),
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determines_range_direction() {
        let range = CoordinateRange::new((0, 0), (0, 5));
        assert_eq!(range.direction(), Direction::Down);

        let range = CoordinateRange::new((0, 0), (5, 0));
        assert_eq!(range.direction(), Direction::Right);

        let range = CoordinateRange::new((0, 5), (0, 0));
        assert_eq!(range.direction(), Direction::Up);

        let range = CoordinateRange::new((5, 0), (0, 0));
        assert_eq!(range.direction(), Direction::Left);
    }

    #[test]
    fn range_intersects_range() {
        let range_1 = CoordinateRange::new((7, 1), (7, 7));
        let range_2 = CoordinateRange::new((8, 6), (2, 6));

        assert_eq!(range_1.intersects(&range_2), true);

        let range_1 = CoordinateRange::new((7, 1), (7, 7));
        let range_2 = CoordinateRange::new((8, 6), (2, 6));

        assert_eq!(range_1.intersects(&range_2), true);
    }
}
