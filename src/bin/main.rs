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
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err(anyhow!("Usage: file ex_num(1-12)"));
    }
    let ex_num = args[1].parse::<u8>()?;

    let data = aoc2025::utils::read_data(ex_num, "data")
        .map_err(|e| anyhow!("Exercise not implemented: {e}"))?;

    match ex_num {
        1 => {
            let mut first = aoc2025::first::Dial::new();
            execute(&mut first, &data);
        }
        2 => {
            let mut second = aoc2025::second::Product::new();
            execute(&mut second, &data);
        }
        _ => {
            return Err(anyhow!("Exercise not implemented"));
        }
    }

    Ok(())
}
