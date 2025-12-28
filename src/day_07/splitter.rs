use crate::util::Position2D;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Map2D {
    splitters: HashMap<Position2D, usize>,
    beams: HashSet<Position2D>,

    x_max: usize,
    y_max: usize,
}

impl Map2D {
    pub fn from_string(input: &str) -> Self {
        let mut splitters = HashMap::new();
        let mut beams = HashSet::new();

        let mut x_max = 0;
        let mut y_max = 0;

        input.lines().enumerate().for_each(|(y, line)| {
            y_max = y;

            line.chars().enumerate().for_each(|(x, character)| {
                x_max = x;

                match character {
                    '^' => {
                        splitters.insert((x, y), 0);
                    }
                    'S' => {
                        beams.insert((x, y));
                    }
                    _ => {}
                };
            });
        });

        Self {
            splitters,
            beams,
            x_max,
            y_max,
        }
    }
}

impl Map2D {
    pub fn count_splits(mut self) -> usize {
        let mut splits = 0;

        while !self.beams.is_empty() {
            let mut entries: Vec<Position2D> = self.beams.drain().collect();

            while let Some(beam) = entries.pop() {
                let (x, y) = beam;
                let new_y = y + 1;
                let new_position = (x, new_y);

                if new_position.1 > self.y_max || new_position.0 > self.x_max {
                    continue;
                }

                let next = self.splitters.get(&new_position);

                match next {
                    None => {
                        self.beams.insert(new_position);
                    }
                    Some(_) => {
                        splits += 1;

                        if x > 0 {
                            let left = (x - 1, new_y);
                            self.beams.insert(left);
                        }

                        let right = (x + 1, new_y);
                        self.beams.insert(right);
                    }
                }
            }
        }

        splits
    }

    fn find_splitter(&self, start: &Position2D) -> Option<(Position2D, usize)> {
        let result = None;

        for y in start.1..self.y_max {
            let position = (start.0, y);
            let splitter = self.splitters.get(&position);

            if splitter.is_some() {
                return splitter.map(|value| (position, *value));
            }
        }

        result
    }

    pub fn count_timelines(mut self) -> usize {
        let beam = self.beams.iter().next().unwrap();
        let first_splitter = self.find_splitter(&beam);

        match first_splitter {
            None => None,
            Some((position, _)) => self.splitters.insert(position, 1),
        };

        for y in 0..self.y_max {
            for x in 0..self.x_max {
                let position = (x, y);
                let splitter = self.splitters.get(&position);

                if splitter.is_none() {
                    continue;
                }

                let total = *splitter.unwrap_or(&0);

                let right = (x + 1, y + 1);
                let next_right = self.find_splitter(&right);

                let next_left = match x > 0 {
                    true => self.find_splitter(&(x - 1, y + 1)),
                    _ => None,
                };

                if let Some((position_right, value)) = next_right {
                    self.splitters.insert(position_right, total + value);
                }

                if let Some((position_left, value)) = next_left {
                    self.splitters.insert(position_left, total + value);
                }
            }
        }

        let sum: usize = self.splitters.into_iter().map(|(_, value)| value).sum();

        sum + 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn counts_map_splits() {
        let input = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............";

        let map = Map2D::from_string(input);

        assert_eq!(map.count_splits(), 21);
    }

    #[test]
    fn counts_map_timelines() {
        let input = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............";

        let map = Map2D::from_string(input);

        assert_eq!(map.count_timelines(), 40);
    }
}
