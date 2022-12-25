use std::borrow::Cow;

use aoc_framework::*;

pub struct Day25;

impl_day!(Day25::{Part1, Part2}: 2022[25], r"
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
");

fn parse_num(s: &str) -> i64 {
    s.bytes().fold(0, |acc, b| {
        acc * 5
            + match b {
                b'=' => -2,
                b'-' => -1,
                b'0'..=b'2' => (b - b'0') as i64,
                _ => panic!("Invalid input"),
            }
    })
}

fn convert(mut n: i64, out: &mut String) {
    if n == 0 {
        return;
    }
    let d = n % 5;
    if d > 2 {
        n += 5;
    }
    n /= 5;
    convert(n, out);
    out.push(match d {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '=',
        4 => '-',
        _ => unreachable!(),
    });
}

pub struct Part1;

impl Part for Part1 {
    type D = Day25;
    const N: u8 = 1;
    const EXAMPLE_RESULT: Option<Answer> = Some(ConstStr("2=-1=0"));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let total = input.map(|s| parse_num(&s)).sum();
        let mut out = String::new();
        convert(total, &mut out);
        Ok(Str(Cow::Owned(out)))
    }
}

pub struct Part2;

impl Part for Part2 {
    type D = Day25;
    const N: u8 = 2;
    const EXAMPLE_RESULT: Option<Answer> = None;

    fn run(_input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        Ok(Num(0))
    }
}
