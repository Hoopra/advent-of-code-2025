#[derive(Debug, PartialEq, Clone, Copy, PartialOrd, Eq, Ord)]
pub enum CoordinateRange {
    X((usize, usize), usize),
    Y(usize, (usize, usize)),
}

impl CoordinateRange {
    pub fn touches(&self, other: &CoordinateRange) -> bool {
        match self {
            CoordinateRange::X(x_range, y) => match other {
                CoordinateRange::X(_, _) => false,
                CoordinateRange::Y(x, y_range) => do_ranges_intersect(x, y_range, x_range, y),
            },
            CoordinateRange::Y(x, y_range) => match other {
                CoordinateRange::X(x_range, y) => do_ranges_intersect(x, y_range, x_range, y),
                CoordinateRange::Y(_, _) => false,
            },
        }
    }
}

fn do_ranges_intersect(
    x: &usize,
    y_range: &(usize, usize),
    x_range: &(usize, usize),
    y: &usize,
) -> bool {
    let (x_min, x_max) = x_range;
    let (y_min, y_max) = y_range;

    if x < x_min || x > x_max || y < y_min || y > y_max {
        return false;
    }

    let touches_x = x == x_min || x == x_max;
    let touches_y = y == y_min || y == y_max;

    !(touches_x && touches_y)
    // {
    //     return false;
    // }

    // return;
    // let intersects_x = x > x_min && x < x_max;
    // let intersects_y = y > y_min && y < y_max;

    // intersects_x || intersects_y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranges_intersect() {
        // .a............
        // bxb...........
        // .a............

        let result = do_ranges_intersect(&1, &(0, 2), &(0, 2), &1);
        assert_eq!(result, true);

        // ..............
        // bxb...........
        // .a............
        // .a............

        let result = do_ranges_intersect(&1, &(1, 3), &(0, 2), &1);
        assert_eq!(result, true);
    }

    #[test]
    fn ranges_overlap_but_do_not_intersect() {
        // ..............
        // .xbbbb........
        // .a............
        // .a............
        // .a............
        // .a............

        let result = do_ranges_intersect(&1, &(1, 5), &(1, 5), &1);
        assert_eq!(result, false);
    }
}
