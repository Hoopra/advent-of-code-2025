// 123 328  51 64
//  45 64  387 23
//   6 98  215 314
// *   +   *   +

#[derive(Clone, Copy, Debug, PartialEq)]
enum Operator {
    Multiply,
    Add,
}

#[derive(PartialEq, Debug)]
pub struct Assignment {
    operator: Operator,
    numbers: Vec<u64>,
}

impl Assignment {
    pub fn multiple_from_string(input: &str) -> Vec<Self> {
        let mut result = vec![];

        let mut operators: Vec<(usize, Operator)> = vec![];

        let operators_line = input.lines().last().unwrap();

        operators_line
            .chars()
            .enumerate()
            .for_each(|(index, character)| {
                match character {
                    '*' => operators.push((index, Operator::Multiply)),
                    '+' => operators.push((index, Operator::Add)),
                    _ => {}
                };
            });

        let lines = input.lines();
        let total_lines = &lines.count();

        operators.iter().for_each(|(index, operator)| {
            let mut numbers: Vec<u64> = vec![];

            input.lines().take(total_lines - 1).for_each(|line| {
                let number: String = line
                    .chars()
                    .skip(*index)
                    .skip_while(|character| character == &' ')
                    .take_while(|character| character != &' ')
                    .collect();

                numbers.push(number.parse().unwrap());
            });

            result.push(Assignment {
                operator: *operator,
                numbers,
            });
        });

        result
    }
}

impl Assignment {
    fn result(&self) -> u64 {
        let start = self.numbers.get(0).unwrap();

        self.numbers
            .iter()
            .skip(1)
            .fold(*start, |result, next| match self.operator {
                Operator::Add => result + next,
                _ => result * next,
            })
    }
}

pub fn assignment_results_from_string(input: &str) -> u64 {
    let assignments = Assignment::multiple_from_string(input);

    assignments.iter().map(Assignment::result).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_assignments_from_text() {
        let expected = vec![
            Assignment {
                operator: Operator::Multiply,
                numbers: vec![123, 45, 6],
            },
            Assignment {
                operator: Operator::Add,
                numbers: vec![328, 64, 98],
            },
            Assignment {
                operator: Operator::Multiply,
                numbers: vec![51, 387, 215],
            },
            Assignment {
                operator: Operator::Add,
                numbers: vec![64, 23, 314],
            },
        ];

        let input = "123 328 51  64 \n45  64  387 23 \n6   98  215 314\n*   +   *   +  ";
        let result = Assignment::multiple_from_string(input);
        assert_eq!(&result, &expected);

        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        let result = Assignment::multiple_from_string(input);
        assert_eq!(&result, &expected);
    }

    #[test]
    fn calculates_result_from_string() {
        let input = "123 328 51  64 \n45  64  387 23 \n6   98  215 314\n*   +   *   +  ";

        let result = assignment_results_from_string(input);

        assert_eq!(result, 4277556);
    }
}
