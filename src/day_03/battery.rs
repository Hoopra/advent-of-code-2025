pub struct BatteryBank {
    batteries: String,
}

impl BatteryBank {
    pub fn from_string(input: &str) -> Self {
        Self {
            batteries: input.to_string(),
        }
    }

    pub fn multiple_from_string(input: String) -> Vec<Self> {
        input.lines().map(BatteryBank::from_string).collect()
    }
}

impl BatteryBank {
    fn best_joltage_two_batteries(bank: &BatteryBank) -> u128 {
        find_maximum_joltage(&bank.batteries, 2)
    }

    fn best_joltage(bank: &BatteryBank, batteries: usize) -> u128 {
        find_maximum_joltage(&bank.batteries, batteries)
    }

    pub fn best_joltages(banks: &Vec<BatteryBank>, batteries: usize) -> u128 {
        banks
            .iter()
            .map(|bank| BatteryBank::best_joltage(bank, batteries))
            .sum()
    }

    pub fn best_joltages_two_batteries(banks: &Vec<BatteryBank>) -> u128 {
        banks
            .iter()
            .map(BatteryBank::best_joltage_two_batteries)
            .sum()
    }
}

fn find_maximum_joltage(numbers: &str, target_length: usize) -> u128 {
    let mut result = numbers.to_string();

    while result.len() > target_length {
        dbg!(&result);
        result = remove_lowest_digit(&result)
    }

    result.parse().unwrap()
}

fn remove_lowest_digit(numbers: &str) -> String {
    let max = highest_digit(numbers);

    let (lowest_index_left, _) = lowest_digit_left(numbers, Some(max));
    let (lowest_index_right, _) = lowest_digit_right(numbers, Some(max));

    let left = remove_digit_at_index(numbers, lowest_index_left);
    let right = remove_digit_at_index(numbers, lowest_index_right);

    let left_parsed = left.parse::<u64>();

    match left_parsed {
        Ok(left_parsed) => match left_parsed > right.parse().unwrap() {
            true => left,
            _ => right,
        },
        Err(_) => left,
    }

    // match left.parse::<u64>().unwrap() > right.parse().unwrap() {
    //     true => left,
    //     _ => right,
    // }
}

fn remove_digit_at_index(entry: &str, index: usize) -> String {
    entry
        .chars()
        .take(index)
        .chain(entry.chars().skip(index + 1))
        .collect()
}

fn highest_digit(numbers: &str) -> u8 {
    numbers.chars().fold(0, |highest, entry| {
        let value: u8 = entry.to_digit(10).unwrap().try_into().unwrap();
        if value > highest { value } else { highest }
    })
}

fn lowest_digit_left(numbers: &str, stop_at: Option<u8>) -> (usize, u8) {
    let stop_at = stop_at.unwrap_or(highest_digit(numbers));

    find_lowest(numbers.chars().enumerate(), stop_at)
}

fn lowest_digit_right(numbers: &str, stop_at: Option<u8>) -> (usize, u8) {
    let stop_at = stop_at.unwrap_or(highest_digit(numbers));
    let max_index = numbers.len() - 1;

    let result = find_lowest(numbers.chars().rev().enumerate(), stop_at);

    (max_index - result.0, result.1)
}

fn find_lowest(chars: impl Iterator<Item = (usize, char)>, stop_at: u8) -> (usize, u8) {
    let mut result = (0, 9);

    for (index, value) in chars {
        let value: u8 = value.to_digit(10).unwrap().try_into().unwrap();

        if value == stop_at {
            return result;
        }

        if value == 1 {
            return (index, value);
        }

        if value < result.1 {
            result = (index, value);
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn finds_maximum_joltage() {
        let result = find_maximum_joltage("987654321111111", 2);
        assert_eq!(result, 98);

        let result = find_maximum_joltage("811111111111119", 2);
        assert_eq!(result, 89);

        let result = find_maximum_joltage("818181911112111", 2);
        assert_eq!(result, 92);

        let result = find_maximum_joltage("987654321111111", 12);
        assert_eq!(result, 987654321111);
    }

    #[test]
    fn finds_lowest_digit() {
        let result = lowest_digit_left("987654321111111", None);
        assert_eq!(result, (0, 9));

        let result = lowest_digit_right("987654321111111", None);
        assert_eq!(result, (14, 1));
    }

    #[test]
    fn removes_lowest_digit() {
        let result = remove_lowest_digit("987654321111111");
        assert_eq!(&result, "98765432111111");
    }
}
