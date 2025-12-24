pub struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }

    pub fn from_string(input: &str) -> Self {
        let mut entries = input.split("-");

        let start = entries.nth(0).unwrap().parse::<u64>().unwrap();
        let end = entries.nth(0).unwrap().parse::<u64>().unwrap();

        Self::new(start, end)
    }

    pub fn many_from_string(input: &str) -> Vec<Self> {
        input.split(",").map(Self::from_string).collect()
    }
}

impl Range {
    pub fn sum_duplicate_once_ids(&self) -> u64 {
        let range = self.start..=self.end;

        range.filter(|entry| is_id_duplicate_once(*entry)).sum()
    }

    pub fn sum_duplicate_at_least_once_ids(&self) -> u64 {
        let range = self.start..=self.end;

        range.filter(|entry| is_id_duplicate(*entry)).sum()
    }

    pub fn sum_duplicate_once_ids_in_ranges(ranges: &[Range]) -> u64 {
        ranges.iter().map(Range::sum_duplicate_once_ids).sum()
    }

    pub fn sum_duplicate_at_least_once_ids_in_ranges(ranges: &[Range]) -> u64 {
        ranges
            .iter()
            .map(Range::sum_duplicate_at_least_once_ids)
            .sum()
    }
}

fn is_id_duplicate_once(entry: u64) -> bool {
    let parts = entry.to_string();
    let length = parts.len();

    if !length.is_multiple_of(2) {
        return false;
    }

    has_identical_parts(&parts, parts.len() / 2)
}

fn is_id_duplicate(entry: u64) -> bool {
    let as_string = entry.to_string();
    let length = as_string.len();
    let half = length / 2;

    for i in 1..=half {
        if !length.is_multiple_of(i) {
            continue;
        }

        if has_identical_parts(&as_string, i) {
            return true;
        }
    }

    false
}

fn has_identical_parts(input: &str, part_length: usize) -> bool {
    let mut i = 0;
    let mut first: Option<&str> = None;

    while i < input.len() {
        let next = &input[i..(i + part_length)];

        if let Some(first) = first
            && !next.eq(first)
        {
            return false;
        }

        first = Some(next);
        i += part_length;
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn determines_if_id_is_duplicate_once() {
        assert_eq!(is_id_duplicate_once(1212), true);
        assert_eq!(is_id_duplicate_once(1122), false);
    }

    #[test]
    fn determines_if_id_is_duplicate_at_least_once() {
        assert_eq!(is_id_duplicate(1212), true);
    }

    #[test]
    fn counts_duplicate_ids() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let ranges = Range::many_from_string(input);

        let result = Range::sum_duplicate_once_ids_in_ranges(&ranges);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn counts_ids_duplicated_at_least_once() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let ranges = Range::many_from_string(input);

        let result = Range::sum_duplicate_at_least_once_ids_in_ranges(&ranges);
        assert_eq!(result, 4174379265);
    }
}
