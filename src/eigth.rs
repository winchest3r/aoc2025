use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::utils::Exercise;

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, Hash)]
pub struct Box {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Box {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    pub fn distance_square(&self, other: &Box) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

#[derive(PartialEq, Eq)]
pub struct Pair {
    pub dist_sq: i64,
    pub first: Box,
    pub second: Box,
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.dist_sq.partial_cmp(&self.dist_sq)
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

pub struct Boxes {
    boxes: Vec<Box>,
    connections: u64,
}

impl Boxes {
    pub fn new() -> Self {
        Self {
            boxes: Vec::new(),
            connections: 1000,
        }
    }

    fn fill(&mut self, data: &str) {
        for line in data.lines() {
            let coords: Vec<i64> = line.trim().split(',').map(|s| s.parse().unwrap()).collect();
            let (x, y, z) = (coords[0], coords[1], coords[2]);
            self.boxes.push(Box::new(x, y, z));
        }
    }

    /// Returns closest pairs.
    fn closest_pairs(&mut self) -> Vec<Pair> {
        let n = self.boxes.len();

        let mut heap = BinaryHeap::new();
        for i in 0..n {
            for j in (i + 1)..n {
                let dist_sq = self.boxes[i].distance_square(&self.boxes[j]);

                let pair = Pair {
                    dist_sq,
                    first: self.boxes[i],
                    second: self.boxes[j],
                };

                heap.push(pair);
            }
        }

        let mut result = heap.into_vec();
        result.sort_by(|a, b| a.dist_sq.partial_cmp(&b.dist_sq).unwrap());
        result
    }

    pub fn connect(&mut self) -> HashMap<Box, Vec<Box>> {
        let mut map = HashMap::new();

        let cons = self.closest_pairs();

        for pair in cons.iter().take(self.connections as usize) {
            map.entry(pair.first)
                .or_insert(Vec::new())
                .push(pair.second);
            map.entry(pair.second)
                .or_insert(Vec::new())
                .push(pair.first);
        }

        map
    }

    fn walk_one_circuit(map: &HashMap<Box, Vec<Box>>, seen: &mut HashSet<Box>, cur: &Box) -> u64 {
        if seen.contains(cur) {
            return 0;
        }
        seen.insert(*cur);
        let mut result = 1;
        for nb in map[cur].iter() {
            result += Boxes::walk_one_circuit(map, seen, nb);
        }
        result
    }

    fn calc_circuits(map: &HashMap<Box, Vec<Box>>) -> Vec<u64> {
        let mut result = Vec::new();
        let mut seen: HashSet<Box> = HashSet::new();
        for entry in map.iter() {
            if seen.contains(&entry.0) {
                continue;
            }
            let value = Boxes::walk_one_circuit(map, &mut seen, entry.0);
            result.push(value);
        }
        result
    }
}

impl Exercise for Boxes {
    fn day(&self) -> u8 {
        8
    }

    fn part1(&mut self, data: &str) -> String {
        self.fill(data);

        let map = self.connect();

        Boxes::calc_circuits(&map)
            .iter()
            .product::<u64>()
            .to_string()
    }

    fn part2(&mut self, data: &str) -> String {
        "unimpl".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::utils::read_data;

    #[test]
    fn eigth_test_one() {
        let data = read_data(8, "test1").unwrap();
        let mut boxes = Boxes::new();
        boxes.connections = 10;
        let result = boxes.part1(&data);
        assert_eq!(result, "40");
    }

    #[test]
    fn eigth_test_two() {
        let data = read_data(8, "test1").unwrap();
        let mut boxes = Boxes::new();
        let result = boxes.part2(&data);
        assert_eq!(result, "unimpl");
    }
}
