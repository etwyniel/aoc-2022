use aoc_framework::*;

pub struct Day3;

impl_day!(Day3::{Part1, Part2}: 2022[3], r"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
");

fn digest(bytes: &[u8]) -> u64 {
    bytes
        .iter()
        .map(|b| match b {
            b'a'..=b'z' => b - b'a',
            b'A'..=b'Z' => b - b'A' + 26,
            _ => panic!("Invalid input"),
        })
        .fold(0, |acc, ndx| acc | 1 << ndx)
}

pub struct Part1;

impl Part for Part1 {
    type D = Day3;
    const N: u8 = 1;
    const EXAMPLE_RESULT: Option<u64> = Some(157);

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<u64> {
        Ok(input
            .map(|line| {
                let (l, r) = line.as_bytes().split_at(line.len() / 2);
                let (l, r) = (digest(l), digest(r));
                (l & r).trailing_zeros() as u64 + 1
            })
            .sum())
    }
}

pub struct Part2;

impl Part for Part2 {
    type D = Day3;
    const N: u8 = 2;
    const EXAMPLE_RESULT: Option<u64> = Some(70);

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<u64> {
        Ok(input
            .chunks(3)
            .into_iter()
            .map(|chunk| {
                chunk
                    .map(|line| digest(line.as_bytes()))
                    .fold(u64::MAX, |acc, n| acc & n)
                    .trailing_zeros() as u64
                    + 1
            })
            .sum())
    }
}
