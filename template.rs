use aoc_framework::*;

pub struct DayN;

impl_day!(DayN: 2022[0], r"");

pub struct Part1;

impl Part for Part1 {
    type D = Day1;
    const N: u8 = 1;
    const EXAMPLE_RESULT: Option<u64> = None;

    fn run(_input: impl Iterator<Item = String>) -> anyhow::Result<u64> {
        Ok(0)
    }
}

pub struct Part2;

impl Part for Part2 {
    type D = Day1;
    const N: u8 = 2;
    const EXAMPLE_RESULT: Option<u64> = None;

    fn run(_input: impl Iterator<Item = String>) -> anyhow::Result<u64> {
        Ok(0)
    }
}

pub fn run(session_key: &str) {
    run_and_display::<Part1>(session_key);
    run_and_display::<Part2>(session_key);
}
