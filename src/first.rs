use super::utils;

pub struct Dial {
    pub value: i32,
}

impl Dial {
    pub fn new() -> Self {
        Self { value: 0 }
    }
}

impl utils::Exercise for Dial {
    fn part1(&mut self, data: &str) -> String {
        self.value = 50;

        let mut left_at_zero = 0;
        for ins in data.lines() {
            let num = ins[1..].parse::<i32>().unwrap();
            let bytes = ins.as_bytes();
            match bytes[0] {
                b'L' => {
                    self.value = (self.value - num).rem_euclid(100);
                }
                b'R' => {
                    self.value = (self.value + num).rem_euclid(100);
                }
                _ => panic!("invalid instruction"),
            }

            if self.value == 0 {
                left_at_zero += 1;
            }
        }

        left_at_zero.to_string()
    }

    fn part2(&mut self, data: &str) -> String {
        self.value = 50;

        let mut clicks = 0;
        for ins in data.lines() {
            let mut num = ins[1..].parse::<i32>().unwrap();
            clicks += num / 100;
            num %= 100;
            let bytes = ins.as_bytes();
            match bytes[0] {
                b'L' => {
                    for _ in 0..num {
                        self.value = (self.value - 1).rem_euclid(100);
                        if self.value == 0 {
                            clicks += 1;
                        }
                    }
                }
                b'R' => {
                    for _ in 0..num {
                        self.value = (self.value + 1).rem_euclid(100);
                        if self.value == 0 {
                            clicks += 1;
                        }
                    }
                }
                _ => panic!("invalid instruction"),
            }
        }

        clicks.to_string()
    }

    fn day(&self) -> u8 {
        1
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::Exercise;

    use super::*;

    #[test]
    fn first_test_one() {
        let mut dial = Dial::new();
        let data = utils::read_data(1, "test1").unwrap();
        let res = dial.part1(&data);
        assert_eq!(res, "3");
    }

    #[test]
    fn first_test_two() {
        let mut dial = Dial::new();
        let data = utils::read_data(1, "test1").unwrap();
        let res = dial.part2(&data);
        assert_eq!(res, "6");
    }

    #[test]
    fn first_test_three() {
        let mut dial = Dial::new();
        let data = utils::read_data(1, "test2").unwrap();
        let res = dial.part2(&data);
        assert_eq!(res, "11");
    }
}
