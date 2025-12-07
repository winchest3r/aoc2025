use std::collections::{HashMap, HashSet};

use crate::utils::Exercise;

pub struct Tachyon {
    field: HashMap<(usize, usize), char>,
    beams: HashSet<(usize, usize)>,
}

impl Tachyon {
    pub fn new() -> Self {
        Self {
            field: HashMap::new(),
            beams: HashSet::new(),
        }
    }

    fn clear(&mut self) {
        self.field.clear();
        self.beams.clear();
    }

    fn fill(&mut self, data: &str) {
        for (i, line) in data.lines().enumerate() {
            for (j, c) in line.trim().chars().enumerate() {
                self.field.insert((i, j), c);
                if c == 'S' {
                    self.beams.insert((i + 1, j));
                }
            }
        }
    }

    /// Returns amount of splits during one step.
    fn step(&mut self) -> u32 {
        let mut splits = 0;
        let next_positions: Vec<_> = self.beams.iter().map(|&(i, j)| (i + 1, j)).collect();
        let mut new_beams = HashSet::new();
        for (i, j) in next_positions {
            if !self.field.contains_key(&(i, j)) {
                continue;
            }

            if self.field[&(i, j)] == '.' {
                new_beams.insert((i, j));
            } else {
                splits += 1;
                let left = (i, j - 1);
                let right = (i, j + 1);
                if self.field.contains_key(&left) && self.field[&left] == '.' {
                    new_beams.insert(left);
                }
                if self.field.contains_key(&right) && self.field[&right] == '.' {
                    new_beams.insert(right);
                }
            }
        }
        self.beams = new_beams;
        splits
    }

    fn inner_timeline(
        &mut self,
        i: usize,
        j: usize,
        mem: &mut HashMap<(usize, usize), u64>,
    ) -> u64 {
        if let Some(&val) = mem.get(&(i, j)) {
            return val;
        }

        if !self.field.contains_key(&(i, j)) {
            mem.insert((i, j), 1);
            return 1;
        }

        let result = if self.field[&(i, j)] == '.' {
            self.inner_timeline(i + 1, j, mem)
        } else {
            self.inner_timeline(i, j - 1, mem) + self.inner_timeline(i, j + 1, mem)
        };

        mem.insert((i, j), result);
        result
    }

    fn quantum_timelines(&mut self) -> u64 {
        let beam = self.beams.iter().next().unwrap();
        let mut mem = HashMap::new();
        return self.inner_timeline(beam.0, beam.1, &mut mem);
    }
}

impl Exercise for Tachyon {
    fn day(&self) -> u8 {
        7
    }

    fn part1(&mut self, data: &str) -> String {
        self.clear();
        self.fill(data);

        let mut splits = 0;
        let mut cur_splits = self.step();
        while !self.beams.is_empty() || cur_splits > 0 {
            splits += cur_splits;
            cur_splits = self.step();
        }

        splits.to_string()
    }

    fn part2(&mut self, data: &str) -> String {
        self.clear();
        self.fill(data);
        self.quantum_timelines().to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn sixth_test_one() {
        let mut tachyon = Tachyon::new();
        let data = utils::read_data(7, "test1").unwrap();
        let res = tachyon.part1(&data);
        assert_eq!(res, "21");
    }

    #[test]
    fn sixth_test_two() {
        let mut tachyon = Tachyon::new();
        let data = utils::read_data(7, "test1").unwrap();
        let res = tachyon.part2(&data);
        assert_eq!(res, "40");
    }
}
