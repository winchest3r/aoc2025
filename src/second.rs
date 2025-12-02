use crate::utils::Exercise;

pub struct Product;

impl Product {
    pub fn new() -> Self {
        Self {}
    }

    fn is_invalid(i: u64) -> bool {
        let i_str = i.to_string();
        let l = i_str.len();
        if l % 2 != 0 {
            return false;
        }
        i_str[..(l / 2)] == i_str[(l / 2)..]
    }

    fn is_invalid_certainly(i: u64) -> bool {
        let i_str = i.to_string();
        let l = i_str.len();
        for w_len in 1..=l / 2 {
            let w: Vec<_> = i_str.as_bytes().chunks(w_len).collect();
            let first = w[0];
            if w.iter().all(|&w| w == first) {
                return true;
            }
        }
        false
    }
}

impl Exercise for Product {
    fn day(&self) -> u8 {
        2
    }

    fn part1(&mut self, data: &str) -> String {
        let ranges = data.trim().split(",").map(|s| {
            let r: Vec<u64> = s.split("-").map(|n| n.parse::<u64>().unwrap()).collect();
            let (start, end) = (r[0], r[1]);
            start..=end
        });
        let mut total_invalid: u64 = 0;
        for r in ranges {
            total_invalid += r.filter(|&i| Product::is_invalid(i)).sum::<u64>();
        }
        total_invalid.to_string()
    }

    fn part2(&mut self, data: &str) -> String {
        let ranges = data.trim().split(",").map(|s| {
            let r: Vec<u64> = s.split("-").map(|n| n.parse::<u64>().unwrap()).collect();
            let (start, end) = (r[0], r[1]);
            start..=end
        });
        let mut total_invalid: u64 = 0;
        for r in ranges {
            total_invalid += r.filter(|&i| Product::is_invalid_certainly(i)).sum::<u64>();
        }
        total_invalid.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn second_test_one() {
        let mut prod = Product::new();
        let data = utils::read_data(2, "test1").unwrap();
        let res_part1 = prod.part1(&data);
        assert_eq!(res_part1, "1227775554");
    }

    #[test]
    fn second_test_two() {
        let mut prod = Product::new();
        let data = utils::read_data(2, "test1").unwrap();
        let res_part1 = prod.part2(&data);
        assert_eq!(res_part1, "4174379265");
    }

    #[test]
    fn second_test_certainty_function_true() {
        for case in [444, 565656, 123123] {
            assert!(Product::is_invalid_certainly(case))
        }
    }
}
