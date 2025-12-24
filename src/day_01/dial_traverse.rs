use super::direction::Direction;

pub struct TraverseZeroDial {
    pub position: i32,
    pub zeroes: u32,
}

impl TraverseZeroDial {
    pub fn new() -> Self {
        Self {
            position: 50,
            zeroes: 0,
        }
    }
}

impl TraverseZeroDial {
    pub fn move_sequence(&mut self, sequence: &str) {
        sequence.lines().for_each(|line| {
            self.move_by_input(line);
        });
    }

    fn move_by_input(&mut self, input: &str) {
        let direction = Direction::from_string(input);
        self.move_by(direction);
    }

    fn traverse_zero(&mut self, direction: &Direction) -> i32 {
        let mut steps = match direction {
            Direction::Right(steps) | Direction::Left(steps) => *steps,
        };

        while steps > 100 {
            steps -= 100;
            self.zeroes += 1;
        }

        steps
    }

    fn move_by(&mut self, direction: Direction) {
        let mut zeroes = 0;
        let current = self.position;
        let started_at_zero = current == 0;
        let steps = self.traverse_zero(&direction);

        let traversed_zero;

        let destination = match direction {
            Direction::Left(_) => {
                let result = current - steps;
                traversed_zero = result <= 0;

                match result > 0 {
                    true => result,
                    false => 100 + result,
                }
            }
            Direction::Right(_) => {
                let result = current + steps;
                traversed_zero = result >= 100;

                match result >= 100 {
                    false => result,
                    true => result - 100,
                }
            }
        };

        if traversed_zero && !started_at_zero {
            zeroes += 1;
        }

        self.zeroes += zeroes;

        if destination == 0 || destination == 100 {
            self.position = 0;
            return;
        }

        self.position = destination;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn moves_dial_by_sequence() {
        let sequence = "L123\nR50\nL40";

        let mut dial = TraverseZeroDial::new();
        dial.move_sequence(sequence);

        assert_eq!(dial.position, 37);
    }

    #[test]
    fn traverses_zero_right() {
        let mut dial = TraverseZeroDial::new();
        dial.move_sequence("R550\nR701");

        assert_eq!(dial.position, 1);
        assert_eq!(dial.zeroes, 13);
    }

    #[test]
    fn traverses_zero_left() {
        let mut dial = TraverseZeroDial::new();
        dial.move_sequence("L550\nL701");

        assert_eq!(dial.position, 99);
        assert_eq!(dial.zeroes, 13);
    }

    #[test]
    fn moves_dial_by_readme_sequence() {
        let sequence = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

        let mut dial = TraverseZeroDial::new();
        dial.move_sequence(sequence);

        assert_eq!(dial.position, 32);
        assert_eq!(dial.zeroes, 6);
    }
}
