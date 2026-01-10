use crate::util::Position2D;

fn to_coordinate(entry: Option<&str>) -> usize {
    entry.unwrap().parse().unwrap()
}

pub fn coordinates_from_string(input: &str) -> Vec<Position2D> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            (to_coordinate(parts.next()), to_coordinate(parts.next()))
        })
        .collect()
}
