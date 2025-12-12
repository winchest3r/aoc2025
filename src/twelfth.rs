use crate::utils::Exercise;

/// all shapes 3x3 fit in u16
/// ```
/// '012'
/// '345'
/// '678'
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Shape {
    pub data: u16,
}

impl From<u16> for Shape {
    fn from(data: u16) -> Self {
        Shape { data }
    }
}

impl Shape {
    /// ```
    /// '###'
    /// '##.'  -> 0b0000000111110110
    /// '##.'
    /// ```
    pub fn new(data: &[&str]) -> Self {
        let mut shape = 0;
        for &row in data {
            for c in row.chars() {
                shape <<= 1;
                shape |= match c {
                    '#' => 1,
                    '.' => 0,
                    _ => panic!("invalid char"),
                }
            }
        }
        Shape { data: shape }
    }

    /// Get cell data
    pub fn get(&self, x: usize, y: usize) -> u8 {
        ((self.data >> (x + y * 3)) & 1) as u8
    }

    /// Left rotation (counterclockwise)
    /// ```
    /// '###'      '#..'
    /// '##.'  ->  '###'
    /// '##.'      '###'
    /// ```
    pub fn rotate_left(&self) -> Shape {
        let d = self.data;
        let mut out = 0;

        out |= ((d >> 2) & 1) << 0; // 2 -> 0
        out |= ((d >> 5) & 1) << 1; // 5 -> 1
        out |= ((d >> 8) & 1) << 2; // 8 -> 2

        out |= ((d >> 1) & 1) << 3; // 1 -> 3
        out |= ((d >> 4) & 1) << 4; // 4 -> 4
        out |= ((d >> 7) & 1) << 5; // 7 -> 5

        out |= ((d >> 0) & 1) << 6; // 0 -> 6
        out |= ((d >> 3) & 1) << 7; // 3 -> 7
        out |= ((d >> 6) & 1) << 8; // 6 -> 8

        out.into()
    }

    /// Flip horizontally.
    /// ```
    /// '###'      '###'
    /// '##.'  ->  '.##'
    /// '##.'      '.##'
    /// ```
    pub fn flip_horizontal(&self) -> Shape {
        let d = self.data;
        let mut out = 0;

        out |= ((d >> 2) & 1) << 0;
        out |= ((d >> 1) & 1) << 1;
        out |= ((d >> 0) & 1) << 2;

        out |= ((d >> 5) & 1) << 3;
        out |= ((d >> 4) & 1) << 4;
        out |= ((d >> 3) & 1) << 5;

        out |= ((d >> 8) & 1) << 6;
        out |= ((d >> 7) & 1) << 7;
        out |= ((d >> 6) & 1) << 8;

        out.into()
    }

    /// Flip vertically.
    /// ```
    /// '###'      '##.'
    /// '##.'  ->  '##.'
    /// '##.'      '###'
    /// ```
    pub fn flip_vertical(&self) -> Shape {
        let d = self.data;
        let mut out = 0;

        out |= ((d >> 6) & 1) << 0;
        out |= ((d >> 7) & 1) << 1;
        out |= ((d >> 8) & 1) << 2;

        out |= ((d >> 3) & 1) << 3;
        out |= ((d >> 4) & 1) << 4;
        out |= ((d >> 5) & 1) << 5;

        out |= ((d >> 0) & 1) << 6;
        out |= ((d >> 1) & 1) << 7;
        out |= ((d >> 2) & 1) << 8;

        out.into()
    }

    pub fn intersects(&self, other: &Shape) -> bool {
        self.data & other.data != 0
    }
}

pub struct Region {
    pub sizes: (usize, usize),
    pub quantities: Vec<usize>,
}

impl Region {
    pub fn new(sizes: (usize, usize), quantities: Vec<usize>) -> Self {
        Self { sizes, quantities }
    }
}

pub struct Final {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

impl Final {
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
            regions: Vec::new(),
        }
    }

    pub fn fill(&mut self, data: &str) {
        let (shapes, regions) = data.trim().rsplit_once("\n\n").unwrap();
        for shape in shapes.trim().split("\n\n") {
            let lines: Vec<&str> = shape.lines().collect();
            self.shapes.push(Shape::new(&lines[1..]))
        }
        for reg in regions.trim().split('\n') {
            let (sizes, quantities) = reg.trim().split_once(": ").unwrap();
            let (x, y) = sizes
                .split_once('x')
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .unwrap();

            let quantities = quantities.split(' ').map(|q| q.parse().unwrap()).collect();
            self.regions.push(Region::new((x, y), quantities));
        }
    }

    pub fn clear(&mut self) {
        self.shapes.clear();
        self.regions.clear();
    }
}

impl Exercise for Final {
    fn day(&self) -> u8 {
        12
    }

    fn part1(&mut self, data: &str) -> String {
        self.clear();
        self.fill(data);

        "unimplemented".to_string()
    }

    fn part2(&mut self, data: &str) -> String {
        self.clear();
        self.fill(data);

        "unimplemented".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::utils::read_data;

    #[test]
    fn twelfth_test_one_part_one() {
        let data = read_data(12, "test1").unwrap();
        let mut fin = Final::new();
        let result = fin.part1(&data);
        assert_eq!(result, "2");
    }

    #[test]
    fn twelfth_test_two_part_two() {
        let data = read_data(12, "test1").unwrap();
        let mut fin = Final::new();
        let result = fin.part1(&data);
        assert_eq!(result, "unimplemented");
    }

    #[test]
    fn twelfth_test_three_check_shape() {
        let shape = Shape::new(&["###", "##.", "##."]);
        let expect_data = 0b0000000111110110;
        assert_eq!(shape.data, expect_data);

        let shape_rotated_once = shape.rotate_left();
        assert_eq!(shape_rotated_once, Shape::new(&["#..", "###", "###"]));

        let shape_flip_hor = shape.flip_horizontal();
        assert_eq!(shape_flip_hor, Shape::new(&["###", ".##", ".##"]));

        let shape_flip_vert = shape.flip_vertical();
        assert_eq!(shape_flip_vert, Shape::new(&["##.", "##.", "###"]));
    }
}
