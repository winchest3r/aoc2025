use std::collections::BTreeMap;

use crate::utils::Exercise;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

pub struct Floor {
    /// (x, y)
    tiles: Vec<Point>,
}

impl Floor {
    pub fn new() -> Self {
        Self { tiles: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.tiles.clear();
    }

    pub fn fill(&mut self, data: &str) {
        for line in data.lines() {
            let (x, y) = line.trim().split_once(',').unwrap();
            self.tiles
                .push(Point::new(x.parse().unwrap(), y.parse().unwrap()));
        }
    }

    pub fn get_rect(a: &Point, b: &Point) -> u64 {
        let width = (b.x - a.x).abs() as u64 + 1;
        let height = (b.y - a.y).abs() as u64 + 1;
        width * height
    }

    /// Builds BTree maps related to vertical and horizontal edges.
    ///
    /// (Tree<x, (y_min, y_max)>, Tree<y, (x_min, x_max)>)
    pub fn build_edges(
        &self,
    ) -> (
        BTreeMap<i64, Vec<(i64, i64)>>,
        BTreeMap<i64, Vec<(i64, i64)>>,
    ) {
        let mut v_map: BTreeMap<i64, Vec<(i64, i64)>> = BTreeMap::new();
        let mut h_map: BTreeMap<i64, Vec<(i64, i64)>> = BTreeMap::new();

        let len = self.tiles.len();
        for i in 0..len {
            let a = self.tiles[i];
            let b = self.tiles[(i + 1) % len]; // wrap around the loop

            if a.x == b.x {
                v_map
                    .entry(a.x)
                    .or_default()
                    .push((a.y.min(b.y), a.y.max(b.y)));
            } else {
                h_map
                    .entry(a.y)
                    .or_default()
                    .push((a.x.min(b.x), a.x.max(b.x)));
            }
        }

        (v_map, h_map)
    }

    /// Rect is valid if it is not cross other edges.
    pub fn rect_is_valid(
        x1: i64,
        x2: i64,
        y1: i64,
        y2: i64,
        v_map: &BTreeMap<i64, Vec<(i64, i64)>>,
        h_map: &BTreeMap<i64, Vec<(i64, i64)>>,
    ) -> bool {
        let (x_min, x_max) = (x1.min(x2), x1.max(x2));
        let (y_min, y_max) = (y1.min(y2), y1.max(y2));

        // check if vertical edges crossing interrior
        for (_x, segs) in v_map.range((x_min + 1)..x_max) {
            for &(y_lo, y_hi) in segs {
                if y_lo < y_max && y_hi > y_min {
                    return false;
                }
            }
        }

        // same for horizontal
        for (_y, segs) in h_map.range((y_min + 1)..y_max) {
            for &(x_lo, x_hi) in segs {
                if x_lo < x_max && x_hi > x_min {
                    return false;
                }
            }
        }

        true
    }

    pub fn largest_rect(&self) -> i64 {
        let (v_map, h_map) = self.build_edges();

        let mut best = 0;
        let len = self.tiles.len();

        for i in 0..len {
            for j in (i + 1)..len {
                let a = self.tiles[i];
                let b = self.tiles[j];

                if a.x == b.x || a.y == b.y {
                    continue;
                }

                let width = (a.x - b.x).abs() + 1;
                let height = (a.y - b.y).abs() + 1;
                let area = width * height;
                if area <= best {
                    continue;
                }

                if Floor::rect_is_valid(a.x, b.x, a.y, b.y, &v_map, &h_map) {
                    best = area;
                }
            }
        }

        best
    }
}

impl Exercise for Floor {
    fn day(&self) -> u8 {
        9
    }

    fn part1(&mut self, data: &str) -> String {
        self.clear();
        self.fill(data);

        let n = self.tiles.len();
        let mut max_area = 0;
        for i in 0..n {
            for j in i + 1..n {
                max_area = max_area.max(Floor::get_rect(&self.tiles[i], &self.tiles[j]));
            }
        }
        max_area.to_string()
    }

    fn part2(&mut self, data: &str) -> String {
        self.clear();
        self.fill(data);

        self.largest_rect().to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::read_data;

    use super::*;

    #[test]
    fn ninth_test_one() {
        let data = read_data(9, "test1").unwrap();
        let mut movie = Floor::new();
        let result = movie.part1(&data);
        assert_eq!(result, "50");
    }

    #[test]
    fn ninth_test_two() {
        let data = read_data(9, "test1").unwrap();
        let mut movie = Floor::new();
        let result = movie.part2(&data);
        assert_eq!(result, "24");
    }
}
