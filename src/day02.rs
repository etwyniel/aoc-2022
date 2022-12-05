use std::{fmt::Debug, str::FromStr};

use aoc_framework::{anyhow::bail, *};

pub struct Day2;

impl_day!(Day2::{Part1, Part2}: 2022[2], r"
A Y
B X
C Z
");

#[repr(i8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum RockPaperScissors {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for RockPaperScissors {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        Ok(match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => bail!("Invalid value {s}"),
        })
    }
}

fn parse_tuple<L: FromStr, R: FromStr>(s: &str) -> (L, R)
where
    L::Err: Debug,
    R::Err: Debug,
{
    let (l, r) = s.split_once(' ').unwrap();
    (l.parse().unwrap(), r.parse().unwrap())
}

pub struct Part1;

impl Part for Part1 {
    type D = Day2;
    const N: u8 = 1;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(15));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        Ok(Num(input
            .map(|line| parse_tuple(&line))
            .map(|(l, r): (RockPaperScissors, RockPaperScissors)| {
                r as i8
                    + match (l as i8 - r as i8).rem_euclid(3) {
                        0 => 3, // draw
                        1 => 0, // loss
                        2 => 6, // win
                        _ => unreachable!(),
                    }
            })
            .map(|v| v as u64)
            .sum()))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum DesiredResult {
    Lose,
    Draw,
    Win,
}

impl FromStr for DesiredResult {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        Ok(match s {
            "X" => DesiredResult::Lose,
            "Y" => DesiredResult::Draw,
            "Z" => DesiredResult::Win,
            _ => bail!("invalid result {s}"),
        })
    }
}

pub struct Part2;

impl Part for Part2 {
    type D = Day2;
    const N: u8 = 2;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(12));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        Ok(Num(input
            .map(|line| parse_tuple(&line))
            .map(|(l, r): (RockPaperScissors, DesiredResult)| match r {
                DesiredResult::Lose => (l as i8 - 2).rem_euclid(3) + 1,
                DesiredResult::Draw => 3 + l as i8,
                DesiredResult::Win => 6 + (l as i8 % 3) + 1,
            })
            .map(|v| v as u64)
            .sum()))
    }
}
