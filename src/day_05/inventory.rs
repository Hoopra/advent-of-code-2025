use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Range {
    start: usize,
    end: usize,
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let result = self.start.cmp(&other.start);

        match result {
            Ordering::Equal => self.end.partial_cmp(&other.end),
            _ => Some(result),
        }
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        let result = self.start.cmp(&other.start);

        match result {
            Ordering::Equal => self.end.cmp(&other.end),
            _ => result,
        }
    }
}

impl Range {
    fn from_string(input: &str) -> Self {
        let mut parts = input.split("-");

        Self {
            start: to_number(parts.next()),
            end: to_number(parts.next()),
        }
    }
}

fn to_number(input: Option<&str>) -> usize {
    input.unwrap().parse().unwrap()
}

impl Range {
    fn is_in_range(&self, number: usize) -> bool {
        number >= self.start && number <= self.end
    }

    fn size(&self) -> usize {
        self.end - self.start + 1
    }

    fn conflate_with(&self, other: &Range) -> Option<Range> {
        if self.start > other.end || other.start > self.end {
            return None;
        }

        if self.start < other.start && self.end > other.end {
            return Some(self.clone());
        }

        if other.start < self.start && other.end > self.end {
            return Some(other.clone());
        }

        let start = std::cmp::min(self.start, other.start);
        let end = std::cmp::max(self.end, other.end);

        Some(Range { start, end })
    }
}

fn ranges_from_string(input: &str) -> Vec<Range> {
    input.lines().map(Range::from_string).collect()
}

pub struct Inventory {
    ranges: Vec<Range>,
}

impl Inventory {
    fn from_string(input: &str) -> Self {
        Self {
            ranges: ranges_from_string(input),
        }
    }
}

impl Inventory {
    fn is_included(&self, number: usize) -> bool {
        self.ranges.iter().any(|range| range.is_in_range(number))
    }

    pub fn total_indices(mut self) -> usize {
        self = self.conflate_ranges();

        self.ranges.iter().fold(0, |sum, next| sum + next.size())
    }

    fn conflate_ranges(mut self) -> Self {
        self.ranges.sort();

        let mut ranges: Vec<Range> = vec![];
        let mut pool = self.ranges;

        while let Some(first) = pool.pop() {
            let mut to_add = vec![];
            let mut was_conflated = false;

            dbg!(&first);

            while let Some(second) = pool.pop() {
                dbg!(&second);
                let conflated = first.conflate_with(&second);

                dbg!(&conflated);

                match conflated {
                    None => {
                        to_add.push(second);
                    }
                    Some(new) => {
                        was_conflated = true;
                        pool.push(new);
                        break;
                    }
                }
            }

            if !was_conflated {
                ranges.push(first);
            }

            pool.append(&mut to_add);

            dbg!(&pool, &ranges);
        }

        Self { ranges }
    }
}

pub fn inventory_from_string(input: &str) -> Inventory {
    let mut parts = input.split("\n\n");

    Inventory::from_string(parts.next().unwrap())
}

pub fn fresh_parts_from_string(input: &str) -> usize {
    let mut parts = input.split("\n\n");

    let inventory = Inventory::from_string(parts.next().unwrap());

    parts
        .next()
        .unwrap()
        .lines()
        .map(|number| number.parse::<usize>().unwrap())
        .filter(|number| inventory.is_included(*number))
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn conflates_inventory_ranges() {
        let input = "3-5\n10-14\n16-20\n12-18";
        let inventory = inventory_from_string(input);

        let conflated = inventory.conflate_ranges();
        assert_eq!(
            conflated.ranges,
            vec![Range::from_string("10-20"), Range::from_string("3-5")]
        );

        let input = "1-1000\n80-200\n210-300\n200-600\n900-1452";
        let inventory = inventory_from_string(input);

        let conflated = inventory.conflate_ranges();
        assert_eq!(conflated.ranges, vec![Range::from_string("1-1452")]);
    }

    #[test]
    fn finds_all_possible_indices() {
        let input = "3-5\n10-14\n16-20\n12-18";
        let inventory = inventory_from_string(input);
        assert_eq!(inventory.total_indices(), 14);

        let input = "1-101\n80-200\n210-300\n200-600";
        let inventory = inventory_from_string(input);
        assert_eq!(inventory.total_indices(), 600);

        let input = "1-1000\n80-200\n210-300\n200-600\n900-1452";
        let inventory = inventory_from_string(input);
        assert_eq!(inventory.total_indices(), 1452);
    }

    #[test]
    fn finds_range_size() {
        let range = Range::from_string("3-5");
        assert_eq!(range.size(), 3);
    }
}
