use anyhow::anyhow;
use aoc_framework::*;

pub struct Day6;

impl_day!(Day6::{Part1, Part2}: 2022[6], r"mjqjpqmgbljsphdztnvjfqwrcgsmlb");

// faster than Itertools::all_unique
fn all_unique(chunk: &[u8]) -> bool {
    chunk
        .iter()
        .fold(0u32, |mask, b| mask | (1 << (*b - b'a') as usize))
        .count_ones() as usize
        == chunk.len()
}

fn first_unique_chunk<const N: usize>(s: &str) -> anyhow::Result<Answer> {
    s.as_bytes()
        .windows(N)
        .enumerate()
        .find(|(_, chunk)| all_unique(chunk))
        .map(|(i, _)| Num(i as u64 + N as u64))
        .ok_or_else(|| anyhow!("No solution found"))
}

pub struct Part1;

impl Part for Part1 {
    type D = Day6;
    const N: u8 = 1;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(7));

    fn run(mut input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        first_unique_chunk::<4>(&input.next().ok_or_else(|| anyhow!("Invalid input"))?)
    }
}

pub struct Part2;

impl Part for Part2 {
    type D = Day6;
    const N: u8 = 2;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(19));

    fn run(mut input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        first_unique_chunk::<14>(&input.next().ok_or_else(|| anyhow!("Invalid input"))?)
    }
}
