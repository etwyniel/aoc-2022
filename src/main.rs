use aoc_framework::*;

mod helpers;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

fn main() -> anyhow::Result<()> {
    let token = std::env::var("AOC_TOKEN")?;

    day01::Day1::run(&token);
    day02::Day2::run(&token);
    day03::Day3::run(&token);
    day04::Day4::run(&token);
    day05::Day5::run(&token);
    day06::Day6::run(&token);
    day07::Day7::run(&token);
    day08::Day8::run(&token);
    day09::Day9::run(&token);
    day10::Day10::run(&token);

    Ok(())
}
