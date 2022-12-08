use std::iter::once;

use aoc_framework::*;

pub struct Day1;

impl_day!(Day1::{Part1, Part2}: 2022[1], r"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
");

pub struct Part1;

impl Part for Part1 {
    type D = Day1;
    const N: u8 = 1;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(24000));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        Ok(Num(input
            .map(|line| line.parse::<u64>().ok())
            // Add empty value to ensure last sum is used
            .chain(once(None))
            .fold((0, 0), |(max, cur), n| match n {
                Some(n) => (max, cur + n),
                None => (max.max(cur), 0),
            })
            .0))
    }
}

pub struct Part2;

impl Part for Part2 {
    type D = Day1;
    const N: u8 = 2;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(45000));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        input
            .chain(once(String::new())) // Add empty line to ensure last value is used
            .try_fold((0, Vec::with_capacity(4)), |(current, mut top), line| {
                if !line.is_empty() {
                    return Ok((current + line.parse::<u64>()?, top));
                }
                // find insertion index
                let (Ok(ndx) | Err(ndx)) = top
                    .binary_search_by_key(&std::cmp::Reverse(current), |&n| std::cmp::Reverse(n));
                top.insert(ndx, current);
                top.truncate(3);
                Ok((0, top))
            })
            .map(|(_, top)| Num(top.iter().sum()))
    }
}
