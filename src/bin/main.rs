use std::time::Instant;

use anyhow::{Result, anyhow};

use aoc2025::{self, utils::Exercise};

fn execute<T>(ex: &mut T, data: &str)
where
    T: Exercise,
{
    println!("-----day {}-----", ex.day());

    let mut now = Instant::now();
    let part1_result = ex.part1(data);
    println!(
        "part 1 result: {} [time elapsed: {}s]",
        part1_result,
        now.elapsed().as_secs_f32()
    );

    now = Instant::now();
    let part2_result = ex.part2(data);
    println!(
        "part 2 result: {} [time elapsed: {}s]",
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

    let data = aoc2025::utils::read_data(ex_num, "data")?;
    match ex_num {
        1 => {
            let mut first = aoc2025::first::Dial::new();
            execute(&mut first, &data);
        }
        _ => {
            return Err(anyhow!("Exercise not implemented"));
        }
    }

    Ok(())
}
