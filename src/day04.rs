use aoc_framework::*;

use crate::helpers::parse_tuple;

pub struct Day4;

impl_day!(Day4::{Part1, Part2}: 2022[4], r"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
");

fn parse_range(s: &str) -> (u64, u64) {
    parse_tuple(s, '-')
}

pub struct Part1;

impl Part for Part1 {
    type D = Day4;
    const N: u8 = 1;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(2));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        Ok(Num(input
            .flat_map(|line| {
                let (a, b) = line.split_once(',')?;
                Some((parse_range(a), parse_range(b)))
            })
            .filter(|((al, ar), (bl, br))| (al <= bl && ar >= br) || (bl <= al && br >= ar))
            .count() as u64))
    }
}

pub struct Part2;

impl Part for Part2 {
    type D = Day4;
    const N: u8 = 2;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(4));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        Ok(Num(input
            .flat_map(|line| {
                let (a, b) = line.split_once(',')?;
                Some((parse_range(a), parse_range(b)))
            })
            .map(|((al, ar), (bl, br))| (al..=ar, bl..=br))
            .filter(|(a, b)| {
                a.contains(b.start())
                    || a.contains(b.end())
                    || b.contains(a.start())
                    || b.contains(a.end())
            })
            .count() as u64))
    }
}
