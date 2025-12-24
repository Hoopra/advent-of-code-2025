#[derive(Debug, PartialEq)]
pub enum Direction {
    Left(i32),
    Right(i32),
}

impl Direction {
    pub fn from_string(input: &str) -> Self {
        let mut chars = input.chars();

        let first = chars.nth(0).unwrap();
        let number_part: String = chars.collect();
        let steps = number_part.parse::<i32>().unwrap();

        match first {
            'L' => Self::Left(steps),
            _ => Self::Right(steps),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn constructs_direction() {
        let input = "L123";
        let direction = Direction::from_string(input);

        assert_eq!(direction, Direction::Left(123));
    }
}
