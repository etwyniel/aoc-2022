use aoc_framework::*;

pub struct DayN;

impl_day!(DayN::{Part1, Part2}: 2022[N], r"");

pub struct Part1;

impl Part for Part1 {
    type D = DayN;
    const N: u8 = 1;
    const EXAMPLE_RESULT: Option<Answer> = None;

    fn run(_input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        Ok(0)
    }
}

pub struct Part2;

impl Part for Part2 {
    type D = DayN;
    const N: u8 = 2;
    const EXAMPLE_RESULT: Option<Answer> = None;

    fn run(_input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        Ok(Num(0))
    }
}
