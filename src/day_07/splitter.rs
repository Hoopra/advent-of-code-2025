use crate::util::Position2D;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Map2D {
    splitters: HashSet<Position2D>,
    beams: HashSet<Position2D>,

    x_max: usize,
    y_max: usize,
}

impl Map2D {
    pub fn from_string(input: &str) -> Self {
        let mut splitters = HashSet::new();
        let mut beams = HashSet::new();

        let mut x_max = 0;
        let mut y_max = 0;

        input.lines().enumerate().for_each(|(y, line)| {
            y_max = y;

            line.chars().enumerate().for_each(|(x, character)| {
                x_max = x;

                match character {
                    '^' => {
                        splitters.insert((x, y));
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

        while self.beams.len() > 0 {
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn counts_map_splits() {
        let input = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............";

        let map = Map2D::from_string(input);
        dbg!(&map);

        assert_eq!(map.count_splits(), 21);
    }
}
