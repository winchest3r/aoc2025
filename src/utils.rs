use anyhow::{Result, anyhow};
use std::{fs::File, io::Read};

/// Main trait for Advent of Code exercises.
pub trait Exercise {
    fn part1(&mut self, data: &str) -> String;
    fn part2(&mut self, data: &str) -> String;
    fn day(&self) -> u8;
}

/// Wrapper function for reading data from a file.
pub fn read_data(ex_num: u8, filename: &str) -> Result<String> {
    if ex_num == 0 || ex_num > 12 {
        return Err(anyhow!("Exercise day must be in range [1, 12]."));
    }

    let mut file = File::open(format!("data/day{ex_num:02}/{filename}"))?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() -> Result<(), anyhow::Error> {
        let data = read_data(1, "test1")?;
        assert!(data.starts_with("L68"));
        Ok(())
    }
}
