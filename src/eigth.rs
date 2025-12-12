use std::collections::HashMap;

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

    pub fn dist_sq(&self, other: &Box) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

#[derive(PartialEq, Eq)]
pub struct Pair {
    pub dist_sq: i64,
    pub first: usize,
    pub second: usize,
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.dist_sq.partial_cmp(&self.dist_sq)
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// Disjoint Set Union
pub struct DSU {
    parent: Vec<usize>,
    size: Vec<u64>,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    // Union nodes if they are not in the same set
    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let pa = self.find(a);
        let pb = self.find(b);

        // if found in both - same set - exit
        if pa == pb {
            return false;
        }

        // union by size
        if self.size[pa] < self.size[pb] {
            self.parent[pa] = pb;
            self.size[pb] += self.size[pa];
        } else {
            self.parent[pb] = pa;
            self.size[pa] += self.size[pb];
        }
        true
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

    /// Returns all pairs
    fn all_pairs(&self) -> Vec<Pair> {
        let n = self.boxes.len();
        let mut result = Vec::new();

        // form all pairs with distances
        for i in 0..n {
            for j in i + 1..n {
                result.push(Pair {
                    dist_sq: self.boxes[i].dist_sq(&self.boxes[j]),
                    first: i,
                    second: j,
                });
            }
        }

        // adn sort them
        result.sort_by(|a, b| {
            a.dist_sq
                .cmp(&b.dist_sq)
                .then(a.first.cmp(&b.first))
                .then(a.second.cmp(&b.second))
        });
        result
    }

    // Unions self.connections closest boxes into DSU
    pub fn connect(&self) -> DSU {
        let n = self.boxes.len();
        let mut dsu = DSU::new(n);
        let pairs = self.all_pairs();

        let mut processed = 0;

        for p in pairs {
            dsu.union(p.first, p.second);
            processed += 1;

            if processed == self.connections {
                break;
            }
        }

        dsu
    }

    // Returns last two boxes merged to graph
    pub fn connect_until_single(&self) -> (usize, usize) {
        let n = self.boxes.len();
        let mut dsu = DSU::new(n);
        let pairs = self.all_pairs();

        let mut merges = 0;

        for p in pairs {
            if dsu.union(p.first, p.second) {
                merges += 1;

                if merges == n - 1 {
                    return (p.first, p.second);
                }
            }
        }
        panic!("bad graph");
    }

    // Get vector with sorted sizes of circuits
    fn calc_circuits(&self, dsu: &mut DSU) -> Vec<u64> {
        let n = self.boxes.len();
        let mut count = HashMap::new();

        for i in 0..n {
            let root = dsu.find(i);
            *count.entry(root).or_insert(0) += 1;
        }

        let mut sizes: Vec<u64> = count.into_values().collect();
        sizes.sort_by(|a, b| b.cmp(a)); // desc
        sizes
    }
}

impl Exercise for Boxes {
    fn day(&self) -> u8 {
        8
    }

    fn part1(&mut self, data: &str) -> String {
        self.fill(data);

        let mut dsu = self.connect();

        self.calc_circuits(&mut dsu)
            .iter()
            .take(3)
            .product::<u64>()
            .to_string()
    }

    fn part2(&mut self, data: &str) -> String {
        self.fill(data);

        let (a, b) = self.connect_until_single();

        (self.boxes[a].x * self.boxes[b].x).to_string()
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
        assert_eq!(result, "25272");
    }
}
