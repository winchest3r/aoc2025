use crate::utils::Exercise;

pub struct Database;

impl Database {
    pub fn new() -> Self {
        Self {}
    }
}

impl Exercise for Database {
    fn day(&self) -> u8 {
        5
    }

    fn part1(&mut self, data: &str) -> String {
        let data: Vec<&str> = data.split("\n\n").collect();

        let mut ranges = Vec::new();
        for range in data[0].split('\n') {
            let (beg, end): (u64, u64) = range
                .split_once('-')
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap();
            ranges.push(beg..=end);
        }

        let mut count = 0;
        for id in data[1].trim().split('\n') {
            let id: u64 = id.parse::<u64>().unwrap();
            if ranges.iter().any(|range| range.contains(&id)) {
                count += 1;
            }
        }

        count.to_string()
    }

    fn part2(&mut self, data: &str) -> String {
        let data: Vec<&str> = data.split("\n\n").collect();

        let mut ranges: Vec<(u64, u64)> = Vec::new();
        for range in data[0].split('\n') {
            let (beg, end): (u64, u64) = range
                .split_once('-')
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap();

            ranges.push((beg, end));
            // sort by start of range then by end
            ranges.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

            // just merge overlapping ranges
            let mut merged = Vec::new();
            let mut curr = ranges[0];
            for &next in ranges.iter().skip(1) {
                // check only beg > end case because we sorted it
                if curr.1 >= next.0 - 1 {
                    curr.1 = curr.1.max(next.1);
                } else {
                    merged.push(curr);
                    curr = next;
                }
            }
            // and replace it after every merge cycle
            merged.push(curr);
            ranges = merged;
        }

        ranges
            .iter()
            .map(|&(s, e)| e - s + 1)
            .sum::<u64>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::read_data;

    use super::*;

    #[test]
    fn fifth_test_one() {
        let mut db = Database::new();
        let data = read_data(5, "test1").unwrap();
        let result = db.part1(&data);
        assert_eq!(result, "3");
    }

    #[test]
    fn fifth_test_two() {
        let mut db = Database::new();
        let data = read_data(5, "test1").unwrap();
        let result = db.part2(&data);
        assert_eq!(result, "14");
    }
}
