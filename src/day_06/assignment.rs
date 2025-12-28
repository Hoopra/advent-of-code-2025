#[derive(Clone, Copy, Debug, PartialEq)]
enum Operator {
    Multiply,
    Add,
}

#[derive(PartialEq, Debug)]
pub struct Assignment {
    operator: Operator,
    numbers: Vec<String>,
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

        let default = (operators_line.len() + 1, Operator::Add);

        operators
            .iter()
            .enumerate()
            .for_each(|(operator_index, (start_index, operator))| {
                let mut numbers: Vec<String> = vec![];

                let (end_index, _) = operators.get(operator_index + 1).unwrap_or(&default);

                input.lines().take(total_lines - 1).for_each(|line| {
                    let number: String = line
                        .chars()
                        .skip(*start_index)
                        .take(*end_index - start_index - 1)
                        .collect();

                    numbers.push(number);
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
        result_with_operator(
            &self
                .numbers
                .iter()
                .map(|entry| entry.trim().parse().unwrap())
                .collect::<Vec<u64>>(),
            &self.operator,
        )
    }

    fn cephalopod_result(&self) -> u64 {
        let as_text: Vec<Vec<char>> = self
            .numbers
            .iter()
            .map(|number| number.to_string().chars().collect())
            .collect();

        let max_length = as_text
            .iter()
            .fold(0, |result, next| std::cmp::max(result, next.len()));

        let mut numbers: Vec<u64> = vec![];

        for index in 0..max_length {
            let mut number = String::new();

            for entry in as_text.iter() {
                let digit = entry.get(index);
                match digit {
                    None => {}
                    Some(digit) => number.push(*digit),
                }
            }

            numbers.push(number.trim().parse().unwrap());
        }

        result_with_operator(&numbers, &self.operator)
    }
}

fn result_with_operator(numbers: &[u64], operator: &Operator) -> u64 {
    let start = numbers.first().unwrap();

    numbers
        .iter()
        .skip(1)
        .fold(*start, |result, next| match operator {
            Operator::Add => result + next,
            _ => result * next,
        })
}

pub fn assignment_results_from_string(input: &str) -> u64 {
    let assignments = Assignment::multiple_from_string(input);

    assignments.iter().map(Assignment::result).sum()
}

pub fn cephalopod_assignment_results_from_string(input: &str) -> u64 {
    let assignments = Assignment::multiple_from_string(input);

    assignments.iter().map(Assignment::cephalopod_result).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_assignments_from_text() {
        let expected = vec![
            Assignment {
                operator: Operator::Multiply,
                numbers: vec![
                    String::from("123"),
                    String::from(" 45"),
                    String::from("  6"),
                ],
            },
            Assignment {
                operator: Operator::Add,
                numbers: vec![
                    String::from("328"),
                    String::from("64 "),
                    String::from("98 "),
                ],
            },
            Assignment {
                operator: Operator::Multiply,
                numbers: vec![
                    String::from(" 51"),
                    String::from("387"),
                    String::from("215"),
                ],
            },
            Assignment {
                operator: Operator::Add,
                numbers: vec![
                    String::from("64 "),
                    String::from("23 "),
                    String::from("314"),
                ],
            },
        ];

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

    #[test]
    fn calculates_cephalopod_result_from_string() {
        let input = "64 \n23 \n314\n+  ";

        let result = cephalopod_assignment_results_from_string(input);
        assert_eq!(result, 1058);

        let input = " 51\n387\n215\n*  ";

        let result = cephalopod_assignment_results_from_string(input);
        assert_eq!(result, 3253600);
    }
}
