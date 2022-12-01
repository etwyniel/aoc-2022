use aoc_framework::*;

mod day01;

fn main() -> anyhow::Result<()> {
    let token = std::env::var("AOC_TOKEN")?;

    day01::run(&token);

    Ok(())
}
