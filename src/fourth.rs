use crate::utils::Exercise;

pub struct Field {
    pub data: Vec<Vec<bool>>,
}

impl Field {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn check_adj(&self, x: i32, y: i32) -> u32 {
        let mut count = 0;

        let directions = [
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
        ];

        for (dy, dx) in directions.iter() {
            let i = y + dy;
            let j = x + dx;

            if i >= 0
                && i < self.data.len() as i32
                && j >= 0
                && j < self.data[i as usize].len() as i32
                && self.data[i as usize][j as usize]
            {
                count += 1;
            }
        }
        count
    }

    fn fill(&mut self, data: &str) {
        self.data.clear();
        for line in data.lines() {
            self.data.push(Vec::new());
            for c in line.chars() {
                self.data.last_mut().unwrap().push(c == '@');
            }
        }
    }
}

impl Exercise for Field {
    fn day(&self) -> u8 {
        4
    }

    fn part1(&mut self, data: &str) -> String {
        const ADJ_ROLLS: u32 = 4;

        self.fill(data);

        let mut total = 0;
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                if self.data[i][j] && self.check_adj(j as i32, i as i32) < ADJ_ROLLS {
                    total += 1;
                }
            }
        }

        total.to_string()
    }

    fn part2(&mut self, data: &str) -> String {
        const ADJ_ROLLS: u32 = 4;

        self.fill(data);

        // remove until we can
        let mut total = 0;
        let mut removed = true;
        while removed {
            removed = false;
            for i in 0..self.data.len() {
                for j in 0..self.data[i].len() {
                    if self.data[i][j] && self.check_adj(j as i32, i as i32) < ADJ_ROLLS {
                        self.data[i][j] = false;
                        total += 1;
                        removed = true;
                    }
                }
            }
        }

        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::read_data;

    use super::*;

    #[test]
    fn fourth_test_one() {
        let mut field = Field::new();
        let data = read_data(field.day(), "test1").unwrap();
        let result = field.part1(&data);
        assert_eq!(result, "13");
    }

    #[test]
    fn fourth_test_two() {
        let mut field = Field::new();
        let data = read_data(field.day(), "test1").unwrap();
        let result = field.part2(&data);
        assert_eq!(result, "43");
    }
    #[test]
    fn fourth_test_three() {
        let mut field = Field::new();
        let data = read_data(field.day(), "test2").unwrap();
        let result = field.part2(&data);
        assert_eq!(result, "9");
    }
}
