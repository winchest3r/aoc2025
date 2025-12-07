use std::time::Instant;

use anyhow::{Result, anyhow};

use aoc2025::{self, utils::Exercise};

fn execute<T>(ex: &mut T, data: &str)
where
    T: Exercise,
{
    let mut now = Instant::now();
    let part1_result = ex.part1(data);
    println!(
        "day_{}\tpart_1\t{}\t{:.3}s",
        ex.day(),
        part1_result,
        now.elapsed().as_secs_f32()
    );

    now = Instant::now();
    let part2_result = ex.part2(data);
    println!(
        "day_{}\tpart_2\t{}\t{:.3}s",
        ex.day(),
        part2_result,
        now.elapsed().as_secs_f32()
    );
}

fn main() -> Result<()> {
    let ex_num = std::env::args()
        .nth(1)
        .ok_or(anyhow!("Usage: file ex_num(1-12)"))?
        .parse::<u8>()?;

    let data = aoc2025::utils::read_data(ex_num, "data")
        .map_err(|e| anyhow!("Exercise not implemented: {e}"))?;

    match ex_num {
        1 => execute(&mut aoc2025::first::Dial::new(), &data),
        2 => execute(&mut aoc2025::second::Product::new(), &data),
        3 => execute(&mut aoc2025::third::JoltageBank::new(), &data),
        4 => execute(&mut aoc2025::fourth::Field::new(), &data),
        5 => execute(&mut aoc2025::fifth::Database::new(), &data),
        6 => execute(&mut aoc2025::sixth::MathProblem::new(), &data),
        7 => execute(&mut aoc2025::seventh::Tachyon::new(), &data),
        _ => return Err(anyhow!("Exercise not implemented")),
    }

    Ok(())
}
