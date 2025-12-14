#[derive(Debug, PartialEq)]
enum Direction {
    Left(i32),
    Right(i32),
}

impl Direction {
    fn from_string(input: &str) -> Self {
        let mut chars = input.chars().into_iter();

        let first = chars.nth(0).unwrap();
        let number_part: String = chars.collect();
        let steps = number_part.parse::<i32>().unwrap();

        match first {
            'L' => Self::Left(steps),
            _ => Self::Right(steps),
        }
    }
}

pub struct Dial {
    pub position: i32,
    pub zeroes: u32,
}

impl Dial {
    pub fn new() -> Self {
        Self {
            position: 50,
            zeroes: 0,
        }
    }
}

impl Dial {
    pub fn move_sequence(&mut self, sequence: &str) {
        sequence.lines().for_each(|line| {
            self.move_by_input(line);
        });
    }

    fn move_by_input(&mut self, input: &str) {
        let direction = Direction::from_string(input);
        self.move_by(direction);
    }

    fn move_by(&mut self, direction: Direction) {
        let current = self.position;

        let destination = match direction {
            Direction::Left(steps) => {
                let result = (current - steps) % 100;

                match result > 0 {
                    true => result,
                    false => 100 + result,
                }
            }
            Direction::Right(steps) => (current + steps) % 100,
        };

        if destination == 0 || destination == 100 {
            self.position = 0;
            self.zeroes += 1;
            return;
        }

        self.position = destination;
    }

    fn move_by_traverse_zero(&mut self, direction: Direction) {
        match direction {
            Direction::Left(steps) => self.move_left_traverse_zero(steps),
            Direction::Right(steps) => self.move_right_traverse_zero(steps),
        }
    }

    fn move_right_traverse_zero(&mut self, steps: i32) {
        let mut steps = steps;

        while steps > 100 {
            steps -= 100;
            self.zeroes += 1;
        }
    }

    fn move_left_traverse_zero(&mut self, steps: i32) {
        let mut steps = steps;

        while steps > 100 {
            steps -= 100;
            self.zeroes += 1;
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

    #[test]
    fn moves_dial_by_sequence() {
        let sequence = "L123\nR50\nL40";

        let mut dial = Dial::new();
        dial.move_sequence(sequence);

        assert_eq!(dial.position, 37);
    }

    #[test]
    fn moves_dial_by_readme_sequence() {
        let sequence = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

        let mut dial = Dial::new();
        dial.move_sequence(sequence);

        assert_eq!(dial.position, 32);
        assert_eq!(dial.zeroes, 3);
    }
}
