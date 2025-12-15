use std::collections::{HashMap, VecDeque};

use good_lp::*;
use regex::Regex;

use crate::utils::Exercise;

#[derive(Clone)]
pub struct Machine {
    /// Bitset related to light switches.
    pub target_lights: u32,
    /// Amount of lights.
    pub len: usize,
    /// Buttons idx.
    pub buttons: Vec<Vec<u32>>,
    /// Button masks.
    pub masks: Vec<u32>,
    /// Joltate data related to every bit.
    pub target_joltage: Vec<u32>,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            target_lights: 0,
            len: 0,
            buttons: Vec::new(),
            masks: Vec::new(),
            target_joltage: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.target_lights = 0;
        self.len = 0;
        self.buttons = Vec::new();
        self.masks = Vec::new();
        self.target_joltage = Vec::new();
    }

    pub fn fill(&mut self, line: &str) {
        let re = Regex::new(r"^\[(?P<l>.*)\] (?P<b>\(.*\)) \{(?P<j>.*)\}$").unwrap();
        if let Some(caps) = re.captures(line.trim()) {
            let (l, b, j) = (&caps["l"], &caps["b"], &caps["j"]);

            // get lights as bits in self.lights
            self.target_lights = l
                .chars()
                .map(|c| match c {
                    '.' => 0,
                    '#' => 1,
                    _ => panic!("invalid data in match"),
                })
                .rev()
                .fold(0, |acc, bit| (acc << 1) | bit);

            self.len = l.len();

            // get buttons as masks for self.lights
            for button in b.split_whitespace() {
                let bits: Vec<u32> = button[1..button.len() - 1]
                    .split(',')
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect();
                let mut mask = 0;
                for bit in &bits {
                    mask |= 1 << bit;
                }
                self.masks.push(mask);
                self.buttons.push(bits);
            }

            for num in j.split(',').map(|s| s.parse::<u32>().unwrap()) {
                self.target_joltage.push(num);
            }
        } else {
            panic!("invalid data in line");
        }
    }

    pub fn get_min_light_swiches(&self) -> u32 {
        // contains presses for selected value
        let mut seen: HashMap<u32, u32> = HashMap::from([(0, 0)]);

        // start with zero
        let mut queue = VecDeque::from([0]);

        // bfs while we don't reach target
        while !queue.is_empty() {
            let cur = queue.pop_front().unwrap();
            let res = *seen.get(&cur).unwrap();

            if cur == self.target_lights {
                return res;
            }

            for mask in self.masks.iter() {
                let val = cur ^ mask;
                if !seen.contains_key(&val) {
                    seen.insert(val, res + 1);
                    queue.push_back(val);
                }
            }
        }

        panic!("must have a solution!")
    }

    pub fn get_min_joltage_inc(&self) -> u64 {
        let m = self.buttons.len();
        let n = self.target_joltage.len();

        let mut vars = variables!();
        let xs: Vec<_> = (0..m)
            .map(|_| vars.add(variable().integer().min(0)))
            .collect();

        let mut model = vars
            .minimise(xs.iter().fold(Expression::from(0.0), |acc, x| acc + *x))
            .using(default_solver);

        for i in 0..n {
            let mut expr: Expression = 0.0.into();

            for (j, btn) in self.buttons.iter().enumerate() {
                if btn.contains(&(i as u32)) {
                    expr += xs[j];
                }
            }

            model = model.with(constraint!(expr == self.target_joltage[i] as f32));
        }

        let solution = model.solve().unwrap();

        xs.iter().map(|x| solution.value(*x) as u64).sum()
    }
}

impl Exercise for Machine {
    fn day(&self) -> u8 {
        10
    }

    fn part1(&mut self, data: &str) -> String {
        self.clear();

        let mut total = 0;
        for line in data.lines() {
            self.fill(line);
            let presses = self.get_min_light_swiches();
            total += presses;
            self.clear();
        }

        total.to_string()
    }

    fn part2(&mut self, data: &str) -> String {
        self.clear();

        let mut total = 0;
        for line in data.lines() {
            self.fill(line);
            let presses = self.get_min_joltage_inc();
            total += presses;
            self.clear();
        }

        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::read_data;

    use super::*;

    #[test]
    fn tenth_test_one() {
        let data = read_data(10, "test1").unwrap();
        let mut machine = Machine::new();
        let result = machine.part1(&data);
        assert_eq!(result, "7");
    }

    #[test]
    fn tenth_test_two() {
        let data = read_data(10, "test1").unwrap();
        let mut machine = Machine::new();
        let result = machine.part2(&data);
        assert_eq!(result, "33");
    }

    #[test]
    fn tenth_test_three() {
        let data = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n";
        let mut machine = Machine::new();
        machine.fill(&data);
        assert_eq!(machine.target_lights, 0b0110);
        assert_eq!(machine.len, 4);
        assert_eq!(
            machine.masks,
            vec![0b1000, 0b1010, 0b0100, 0b1100, 0b0101, 0b0011]
        );
        assert_eq!(machine.target_joltage, vec![3, 5, 4, 7]);
        assert_eq!(machine.get_min_light_swiches(), 2);
    }

    #[test]
    fn tenth_test_four() {
        let data = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let mut machine = Machine::new();
        machine.fill(&data);
        assert_eq!(machine.target_lights, 0b01000);
        assert_eq!(machine.len, 5);
        assert_eq!(
            machine.masks,
            vec![0b11101, 0b01100, 0b10001, 0b00111, 0b11110]
        );
        assert_eq!(machine.target_joltage, vec![7, 5, 12, 7, 2]);
        assert_eq!(machine.get_min_light_swiches(), 3);
    }
}
