#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    pub fn all() -> [Direction; 8] {
        [
            Direction::N,
            Direction::NE,
            Direction::E,
            Direction::SE,
            Direction::S,
            Direction::SW,
            Direction::W,
            Direction::NW,
        ]
    }

    pub fn corner() -> [Direction; 4] {
        [Direction::NE, Direction::SE, Direction::SW, Direction::NW]
    }

    pub fn cardinal() -> [Direction; 4] {
        [Direction::N, Direction::E, Direction::W, Direction::S]
    }
}

impl Direction {
    pub fn step_2d(&self) -> (isize, isize) {
        match self {
            Direction::N => (0, -1),
            Direction::NE => (1, -1),
            Direction::E => (1, 0),
            Direction::SE => (1, 1),
            Direction::S => (0, 1),
            Direction::SW => (-1, 1),
            Direction::W => (-1, 0),
            Direction::NW => (-1, -1),
        }
    }

    pub fn rotate_90_degress_clockwise(&self) -> Direction {
        match self {
            Direction::N => Direction::E,
            Direction::NE => Direction::SE,
            Direction::E => Direction::S,
            Direction::SE => Direction::SW,
            Direction::S => Direction::W,
            Direction::SW => Direction::NW,
            Direction::W => Direction::N,
            Direction::NW => Direction::NE,
        }
    }

    pub fn rotate_90_degress_counter_clockwise(&self) -> Direction {
        match self {
            Direction::N => Direction::W,
            Direction::NE => Direction::SW,
            Direction::E => Direction::N,
            Direction::SE => Direction::NE,
            Direction::S => Direction::E,
            Direction::SW => Direction::SE,
            Direction::W => Direction::S,
            Direction::NW => Direction::SW,
        }
    }
}
