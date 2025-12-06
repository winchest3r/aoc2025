use crate::utils::Exercise;

pub struct MathProblem {
    data: Vec<Vec<u64>>,
    ops: Vec<char>,
}

impl MathProblem {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            ops: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.ops.clear();
    }

    fn fill(&mut self, data: &str) {
        let first_line = data.lines().next().unwrap();
        for num in first_line.split_whitespace() {
            let num: u64 = num.parse().unwrap();
            self.data.push(vec![num]);
        }

        for line in data.lines().skip(1) {
            if line.starts_with(&['+', '*']) {
                self.ops.extend(
                    line.trim()
                        .split_whitespace()
                        .map(|s| s.chars().nth(0).unwrap()),
                );
            } else {
                for (i, num) in line
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .enumerate()
                {
                    self.data[i].push(num);
                }
            }
        }
    }

    fn cephalopod_fill(&mut self, data: &str) {
        let width = data.lines().next().unwrap().len() + 1; // +'\n'
        let idx = |i: usize, j: usize| -> usize { i * width + j };

        let mut row = 0;
        let mut col = 0;

        // get all areas with numbers
        let bytes = data.as_bytes();
        while col < width {
            let beg_row = 0;
            let beg_col = col;

            // process row till the operator
            while ![b'*', b'+'].contains(&bytes[idx(row, col)]) {
                row += 1;
            }
            self.ops.push(bytes[idx(row, col)] as char);

            // end of the area when bytes in all rows are spaces or '\n'
            let mut has_digit = true;
            while has_digit {
                col += 1;
                has_digit = false;
                for i in 0..row {
                    if bytes[idx(i, col)].is_ascii_digit() {
                        has_digit = true;
                    }
                }
            }

            // read numbers row by row
            self.data.push(Vec::new());
            for j in beg_col..col {
                let mut num = 0;
                for i in beg_row..row {
                    if bytes[idx(i, j)].is_ascii_digit() {
                        num = num * 10 + (bytes[idx(i, j)] - b'0') as u64;
                    }
                }
                self.data.last_mut().unwrap().push(num);
            }

            col += 1;
        }
    }

    fn do_math(&mut self) -> u64 {
        let mut total = 0;
        for (i, op) in self.ops.iter().enumerate() {
            let value: u64 = match op {
                '+' => self.data[i].iter().sum(),
                '*' => self.data[i].iter().product(),
                _ => panic!("invalid op"),
            };
            total += value;
        }
        total
    }
}

impl Exercise for MathProblem {
    fn day(&self) -> u8 {
        6
    }

    fn part1(&mut self, data: &str) -> String {
        self.clear();
        self.fill(data);
        self.do_math().to_string()
    }

    fn part2(&mut self, data: &str) -> String {
        self.clear();
        self.cephalopod_fill(data);
        self.do_math().to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn sixth_test_one() {
        let mut problem = MathProblem::new();
        let data = utils::read_data(6, "test1").unwrap();
        let res = problem.part1(&data);
        assert_eq!(res, "4277556");
    }

    #[test]
    fn sixth_test_two() {
        let mut problem = MathProblem::new();
        let data = utils::read_data(6, "test1").unwrap();
        let res = problem.part2(&data);
        assert_eq!(res, "3263827");
    }
}
