use std::collections::HashSet;
use std::ops::Add;

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

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub struct Bounds {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
}

impl Bounds {
    pub fn contains(&self, p: &Point) -> bool {
        p.x >= self.x_min && p.x <= self.x_max && p.y >= self.y_min && p.y <= self.y_max
    }
}

pub struct Floor {
    /// (x, y)
    tiles: Vec<Point>,
    shape: HashSet<Point>,
}

impl Floor {
    pub fn new() -> Self {
        Self {
            tiles: Vec::new(),
            shape: HashSet::new(),
        }
    }

    pub fn clear(&mut self) {
        self.tiles.clear();
        self.shape.clear();
    }

    pub fn fill(&mut self, data: &str) {
        for line in data.lines() {
            let (x, y) = line.trim().split_once(',').unwrap();
            self.tiles
                .push(Point::new(x.parse().unwrap(), y.parse().unwrap()));
        }
    }

    fn get_bounds(&self) -> Bounds {
        let x_min = self.tiles.iter().map(|p| p.x).min().unwrap() - 1;
        let x_max = self.tiles.iter().map(|p| p.x).max().unwrap() + 1;
        let y_min = self.tiles.iter().map(|p| p.y).min().unwrap() - 1;
        let y_max = self.tiles.iter().map(|p| p.y).max().unwrap() + 1;

        Bounds {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    pub fn form_shape(&mut self) {
        let bounds = self.get_bounds();

        // fill corners
        for &p in self.tiles.iter() {
            for dir in [
                Point::new(1, 0),
                Point::new(0, 1),
                Point::new(-1, 0),
                Point::new(0, -1),
            ] {
                let mut out_of_range = false;
                let mut points = vec![p];
                // check direction - if out of range - ignore
                while !out_of_range {
                    let next = *points.last().unwrap() + dir;
                    points.push(next);
                    if !bounds.contains(&next) {
                        out_of_range = true;
                    // or add it to shape
                    } else if self.tiles.contains(&next) {
                        self.shape.extend(points);
                        break;
                    }
                }
            }
        }

        // fill inside
        let corners: HashSet<_> = self.shape.iter().copied().collect();
        for y in bounds.y_min..bounds.y_max {
            let mut inside = false;
            for x in bounds.x_min..bounds.x_max {
                let point = Point::new(x, y);

                // hitting a corner - toggle
                if corners.contains(&point) {
                    inside = true;
                } else if inside {
                    self.shape.insert(point);
                }
            }
        }
    }

    pub fn get_rect(a: &Point, b: &Point) -> u64 {
        let width = (b.x - a.x).abs() as u64 + 1;
        let height = (b.y - a.y).abs() as u64 + 1;
        width * height
    }

    pub fn get_rect_set(a: &Point, b: &Point) -> HashSet<Point> {
        let mut rect = HashSet::new();

        let x_min = a.x.min(b.x);
        let x_max = a.x.max(b.x);
        let y_min = a.y.min(b.y);
        let y_max = a.y.max(b.y);

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                rect.insert(Point::new(x, y));
            }
        }

        rect
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
        self.form_shape();

        let n = self.tiles.len();
        let mut max_area = 0;
        for i in 0..n {
            for j in i + 1..n {
                let rect = Floor::get_rect_set(&self.tiles[i], &self.tiles[j]);
                if rect.is_subset(&self.shape) {
                    max_area = max_area.max(rect.len());
                }
            }
        }

        max_area.to_string()
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
