pub struct BatteryBank {
    batteries: String,
}

impl BatteryBank {
    pub fn from_string(input: &str) -> Self {
        Self {
            batteries: input.to_string(),
        }
    }

    pub fn multiple_from_string(input: &str) -> Vec<Self> {
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
    let mut indices: Vec<usize> = vec![];

    while indices.len() < target_length {
        dbg!(&indices);
        let found = highest_digit_index(numbers, Some(&indices));

        match found {
            None => break,
            Some(found_index) => {
                indices.push(found_index);
            }
        }
    }

    indices.sort();

    let result: String = numbers
        .chars()
        .enumerate()
        .filter_map(|(index, entry)| match indices.contains(&index) {
            true => Some(entry),
            _ => None,
        })
        .collect();

    result.parse().unwrap()
}

fn highest_digit_index(numbers: &str, indices: Option<&Vec<usize>>) -> Option<usize> {
    let indices = match indices {
        Some(value) => value,
        None => &vec![],
    };

    let start_at = *indices.last().unwrap_or(&0);
    let mut index = start_at.clone();
    let mut steps = 0;

    let mut result: Option<(usize, u8)> = None;
    let mut has_value_left = false;

    let numbers: Vec<u8> = numbers
        .chars()
        .map(|entry| entry.to_digit(10).unwrap().try_into().unwrap())
        .collect();

    while steps < numbers.len() {
        if indices.contains(&index) {
            index += 1;
            steps += 1;
            has_value_left = true;
            continue;
        }

        if index >= numbers.len() {
            if result.is_some() {
                return result.map(|(index, _)| index);
            }

            has_value_left = false;
            index = 0;
            continue;
        }

        let value = *numbers.get(index).unwrap();

        if value == 9 {
            return Some(index);
        }

        match result {
            None => result = Some((index, value)),
            Some((_, result_value)) => {
                if value > result_value || (has_value_left && value >= result_value) {
                    result = Some((index, value));
                }
            }
        }

        steps += 1;
        index += 1;
    }

    result.map(|(index, _)| index)
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
    }

    #[test]
    fn finds_maximum_joltage_12_batteries() {
        let result = find_maximum_joltage("987654321111111", 12);
        assert_eq!(result, 987654321111);

        let result = find_maximum_joltage("811111111111119", 12);
        assert_eq!(result, 811111111119);

        let result = find_maximum_joltage("234234234234278", 12);
        assert_eq!(result, 434234234278);

        let result = find_maximum_joltage("818181911112111", 12);
        assert_eq!(result, 888911112111);
    }

    #[test]
    fn finds_correct_highest_indices() {
        let result = highest_digit_index("234234234234278", None);
        assert_eq!(result, Some(14));

        let result = highest_digit_index("234234234234278", Some(&vec![14]));
        assert_eq!(result, Some(13));

        let result = highest_digit_index("234234234234278", Some(&vec![14, 13]));
        assert_eq!(result, Some(2));

        let result = highest_digit_index("234234234234278", Some(&vec![14, 13, 2]));
        assert_eq!(result, Some(11));

        let result = highest_digit_index("234234234234278", Some(&vec![14, 13, 2, 11]));
        assert_eq!(result, Some(12));

        let result = highest_digit_index("234234234234278", Some(&vec![14, 13, 2, 5, 11, 12]));
        assert_eq!(result, Some(8));

        let result = highest_digit_index("234234234234278", Some(&vec![14, 13, 2, 5, 11, 12, 8]));
        assert_eq!(result, Some(10));
    }

    #[test]
    fn finds_highest_digit_index() {
        let result = highest_digit_index("987654321111111", None);
        assert_eq!(result, Some(0));

        let result = highest_digit_index("987654321111111", Some(&vec![0]));
        assert_eq!(result, Some(1));

        let result = highest_digit_index("987654321111111", Some(&vec![3]));
        assert_eq!(result, Some(4));

        let result = highest_digit_index("111111123456789", Some(&vec![14]));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn sums_joltages() {
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";

        let banks = BatteryBank::multiple_from_string(input);
        let result = BatteryBank::best_joltages(&banks, 12);

        assert_eq!(result, 3121910778619);
    }
}
