use crate::util::Position2D;
use std::collections::HashMap;

pub struct Map {
    occupied: HashMap<Position2D, bool>,
}

impl Map {
    pub fn from_string(input: &str) -> Self {
        let mut occupied = HashMap::new();

        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, character)| {
                if character == '@' {
                    occupied.insert((x, y), true);
                }
            });
        });

        Self { occupied }
    }
}

impl Map {
    pub fn rolls_with_max_neighbors(&self, max: usize) -> usize {
        self.occupied
            .iter()
            .filter(|(position, _)| self.occupied_neighbors(position) < max)
            .count()
    }

    pub fn occupied_neighbors(&self, position: &Position2D) -> usize {
        let neighbors = neighboring_positions(position);

        neighbors
            .iter()
            .filter(|position| self.occupied.contains_key(position))
            .count()
    }

    pub fn remove_all_accessible_rolls(&mut self, max: usize) -> usize {
        let mut total_removed = 0;

        loop {
            let removed = self.remove_accessible_rolls(max);
            total_removed += removed;

            if removed == 0 {
                break;
            }
        }

        total_removed
    }

    pub fn remove_accessible_rolls(&mut self, max: usize) -> usize {
        let to_remove: Vec<Position2D> = self
            .occupied
            .iter()
            .filter_map(|(position, _)| {
                if self.occupied_neighbors(position) < max {
                    return Some(*position);
                }

                None
            })
            .collect();

        to_remove.iter().for_each(|position| {
            self.occupied.remove(position);
        });

        to_remove.len()
    }
}

fn neighboring_positions(position: &Position2D) -> Vec<Position2D> {
    let (x, y) = *position;

    let mut result = vec![(x, y + 1), (x + 1, y), (x + 1, y + 1)];

    if x > 0 {
        result.push((x - 1, y));
        result.push((x - 1, y + 1));
    }

    if y > 0 {
        result.push((x, y - 1));
        result.push((x + 1, y - 1));
    }

    if y > 0 && x > 0 {
        result.push((x - 1, y - 1));
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn finds_paper_rolls_with_max_neighbors() {
        let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

        let map = Map::from_string(input);
        let result = map.rolls_with_max_neighbors(4);

        assert_eq!(result, 13);
    }

    #[test]
    fn removes_all_accessible_rolls() {
        let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

        let mut map = Map::from_string(input);
        let result = map.remove_all_accessible_rolls(4);

        assert_eq!(result, 43);
    }
}
