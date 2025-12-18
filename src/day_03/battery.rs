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
    let mut result = String::new();

    let mut start_index = 0;
    let mut end_index = numbers.len() - target_length;

    while result.len() < target_length {
        let search = &numbers[start_index..=end_index];
        let (index, value) = find_highest_digit(search);

        result.push_str(&value.to_string());

        start_index += index + 1;
        end_index = numbers.len() - target_length + result.len();
    }

    result.parse().unwrap()
}

fn find_highest_digit(numbers: &str) -> (usize, u8) {
    let mut highest = (0, 0);

    for (index, entry) in numbers.chars().enumerate() {
        let value: u8 = entry.to_digit(10).unwrap().try_into().unwrap();

        if value == 9 {
            return (index, value);
        }

        if value > highest.1 {
            highest = (index, value);
        }
    }

    highest
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
}
