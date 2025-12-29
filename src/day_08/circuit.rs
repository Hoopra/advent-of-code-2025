use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Hash)]
pub struct Junction {
    x: u64,
    y: u64,
    z: u64,
}

fn to_coordinate(entry: Option<&str>) -> u64 {
    entry.unwrap().parse().unwrap()
}

fn coordinate_square(a: u64, b: u64) -> u64 {
    let a = a as i64;
    let b = b as i64;

    (b - a).pow(2) as u64
}

fn sorted_identities(a: &str, b: &str) -> String {
    let mut identities = vec![a, b];
    identities.sort();

    let mut iterator = identities.into_iter();
    format!("{}-{}", iterator.next().unwrap(), iterator.next().unwrap())
}

impl Junction {
    fn from_string(input: &str) -> Self {
        let mut entries = input.split(',');

        Self {
            x: to_coordinate(entries.next()),
            y: to_coordinate(entries.next()),
            z: to_coordinate(entries.next()),
        }
    }

    fn identity(&self) -> String {
        format!("{},{},{}", self.x, self.y, self.z)
    }

    pub fn many_from_string(input: &str) -> Vec<Self> {
        input.lines().map(Self::from_string).collect()
    }
}

impl Junction {
    fn distance_to(&self, other: &Self) -> f64 {
        let squared = coordinate_square(self.x, other.x)
            + coordinate_square(self.y, other.y)
            + coordinate_square(self.z, other.z);

        squared as f64
    }
}

fn junction_distances(junctions: &[Junction]) -> HashMap<String, f64> {
    let mut distances: HashMap<String, f64> = HashMap::new();

    for junction in junctions.iter() {
        let identity = junction.identity();

        for other in junctions.iter() {
            let other_identity = other.identity();

            if identity == other_identity {
                continue;
            }

            let key = sorted_identities(&identity, &other_identity);

            if distances.contains_key(&key) {
                continue;
            }

            let distance = junction.distance_to(other);
            distances.insert(key, distance);
        }
    }

    distances
}

pub fn connect_junctions(junctions: &Vec<Junction>, connections: usize) -> HashMap<String, u64> {
    let distances = junction_distances(&junctions);
    let mut distances: Vec<(String, f64)> = distances.into_iter().collect();

    distances.sort_by(|(_, distance_a), (_, distance_b)| {
        let result = distance_b.partial_cmp(distance_a);
        result.unwrap_or(Ordering::Equal)
    });

    let mut circuits: HashMap<String, u64> = HashMap::new();
    let mut next_circuit = 0;
    let mut connected = 0;

    while let Some((key, _)) = distances.pop()
        && connected < connections
    {
        let mut parts = key.split('-');
        let key_1 = parts.next().unwrap();
        let key_2 = parts.next().unwrap();

        let circuit_1 = circuits.get(key_1);
        let circuit_2 = circuits.get(key_2);

        connected += 1;

        if circuit_1.is_some() && circuit_2.is_some() {
            let circuit_1 = *circuit_1.unwrap();
            let circuit_2 = *circuit_2.unwrap();

            let found: Vec<String> = circuits
                .iter()
                .filter_map(|(key, circuit)| match circuit == &circuit_2 {
                    true => Some(key.to_string()),
                    false => None,
                })
                .collect();

            for key in found {
                circuits.insert(key.clone(), circuit_1);
            }

            continue;
        }

        let target_circuit;

        if circuit_1.is_none() && circuit_2.is_none() {
            target_circuit = next_circuit;
            next_circuit += 1;
        } else {
            target_circuit = *circuit_1.unwrap_or(circuit_2.unwrap_or(&0));
        }

        circuits.insert(key_1.to_string(), target_circuit);
        circuits.insert(key_2.to_string(), target_circuit);
    }

    circuits
}

pub fn flatten_circuits(circuits: &HashMap<String, u64>) -> Vec<u64> {
    let circuit_ids: Vec<&u64> = circuits.values().collect();

    let mut circuit_lengths: HashMap<u64, u64> = HashMap::new();

    for id in circuit_ids {
        let mut count = 0;

        for value in circuits.values() {
            if value == id {
                count += 1;
            }
        }

        circuit_lengths.insert(*id, count);
    }

    let mut entries = circuit_lengths
        .values()
        .map(|value| *value)
        .collect::<Vec<u64>>();

    entries.sort_by(|a, b| b.cmp(a));

    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connects_junctions() {
        let input = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689";

        let junctions = Junction::many_from_string(input);
        let result = connect_junctions(&junctions, 10);

        assert_eq!(
            result,
            HashMap::from([
                ("425,690,689".to_string(), 0),
                ("162,817,812".to_string(), 0),
                ("346,949,466".to_string(), 0),
                ("431,825,988".to_string(), 0),
                ("862,61,35".to_string(), 1),
                ("739,650,466".to_string(), 1),
                ("805,96,715".to_string(), 1),
                ("906,360,560".to_string(), 1),
                ("984,92,344".to_string(), 1),
                ("117,168,530".to_string(), 3),
                ("52,470,668".to_string(), 3),
                ("819,987,18".to_string(), 4),
                ("941,993,340".to_string(), 4),
            ])
        );
    }

    #[test]
    fn determines_circuit_length_distribution() {
        let input = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689";

        let junctions = Junction::many_from_string(input);
        let circuits = connect_junctions(&junctions, 10);
        let lengths = flatten_circuits(&circuits);

        assert_eq!(lengths, vec![5, 4, 2, 2]);
    }
}
