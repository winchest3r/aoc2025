use crate::utils::Exercise;
use itertools::Itertools;

pub struct JoltageBank;

impl JoltageBank {
    pub fn new() -> Self {
        Self {}
    }

    /// ~O(digs.len()^max_digits)
    ///
    /// Really slow for max_digits > 5
    pub fn max_joltage_slow(digs: &[u8], max_digits: u64) -> u64 {
        digs.iter()
            .copied()
            .combinations(max_digits as usize)
            .map(|comb| comb.iter().fold(0, |acc, &dig| acc * 10 + dig as u64))
            .max()
            .unwrap_or(0)
    }

    /// O(digs.len())
    ///
    /// Walks through data and removes smaller elements
    /// to get maximum possible joltage.
    pub fn max_joltage_fast(digs: &[u8], max_digits: u64) -> u64 {
        let n = digs.len();
        let k = max_digits as usize;

        let mut stack = Vec::with_capacity(k);
        let mut to_remove = n - k;

        for &d in digs {
            while to_remove > 0 && !stack.is_empty() && *stack.last().unwrap() < d {
                stack.pop();
                to_remove -= 1;
            }
            stack.push(d);
        }

        stack.truncate(k);

        stack.iter().fold(0, |acc, &d| 10 * acc + d as u64)
    }
}

impl Exercise for JoltageBank {
    fn day(&self) -> u8 {
        3
    }

    fn part1(&mut self, data: &str) -> String {
        let mut result = 0;
        for line in data.lines() {
            let digits: Vec<_> = line.trim().as_bytes().iter().map(|&b| b - b'0').collect();
            result += JoltageBank::max_joltage_slow(&digits, 2);
        }
        result.to_string()
    }

    fn part2(&mut self, data: &str) -> String {
        let mut result = 0;
        for line in data.lines() {
            let digits: Vec<_> = line.trim().as_bytes().iter().map(|&b| b - b'0').collect();
            result += JoltageBank::max_joltage_fast(&digits, 12);
        }
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn third_test_one() {
        let mut bank = JoltageBank::new();
        let data = utils::read_data(3, "test1").unwrap();
        let res = bank.part1(&data);
        assert_eq!(res, "357");
    }

    #[test]
    fn third_test_two() {
        let mut bank = JoltageBank::new();
        let data = utils::read_data(3, "test1").unwrap();
        let res = bank.part2(&data);
        assert_eq!(res, "3121910778619");
    }
}
