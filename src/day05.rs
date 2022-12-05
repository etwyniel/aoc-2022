use std::borrow::Cow;

use aoc_framework::{
    anyhow::{anyhow, bail},
    *,
};

pub struct Day5;

impl_day!(Day5::{Part1, Part2}: 2022[5], r"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
");

fn parse_stacks(it: &mut impl Iterator<Item = String>) -> anyhow::Result<Vec<Vec<u8>>> {
    let mut it = it.peekable();
    let first = it.peek().ok_or_else(|| anyhow!("Invalid input"))?;
    let n = (first.len() + 1) / 4;
    let mut stacks = vec![Vec::new(); n];
    'outer: for line in it {
        for (stack, c) in stacks.iter_mut().zip(line.bytes().skip(1).step_by(4)) {
            match c {
                b' ' => continue,
                b'0'..=b'9' => break 'outer,
                b'A'..=b'Z' => stack.push(c),
                invalid => bail!("Invalid input: incorrect character {invalid:?}"),
            }
        }
    }
    stacks.iter_mut().for_each(|stack| stack.reverse());
    Ok(stacks)
}

fn stacks_result(stacks: &[Vec<u8>]) -> String {
    let num_stacks = stacks.len();
    stacks
        .iter()
        .fold(String::with_capacity(num_stacks), |mut out, stack| {
            out.extend(stack.last().map(|b| *b as char));
            out
        })
}

fn parse_instruction(line: &str) -> Option<(usize, usize, usize)> {
    line.split(' ')
        .skip(1)
        .step_by(2)
        .map(|s| s.parse().unwrap())
        .collect_tuple()
}

pub struct Part1;

impl Part for Part1 {
    type D = Day5;
    const N: u8 = 1;
    const EXAMPLE_RESULT: Option<Answer> = Some(ConstStr("CMZ"));

    fn run(mut input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let mut stacks = parse_stacks(&mut input)?;
        input
            .flat_map(|line| parse_instruction(&line))
            .for_each(|(n, src, dst)| {
                for _ in 0..n {
                    let val = stacks[src - 1].pop();
                    stacks[dst - 1].extend(val);
                }
            });
        Ok(Str(Cow::Owned(stacks_result(&stacks))))
    }
}

pub struct Part2;

impl Part for Part2 {
    type D = Day5;
    const N: u8 = 2;
    const EXAMPLE_RESULT: Option<Answer> = Some(ConstStr("MCD"));

    fn run(mut input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let mut stacks = parse_stacks(&mut input)?;
        let mut refs = stacks.iter_mut().map(Some).collect::<Vec<_>>();
        input
            .flat_map(|line| parse_instruction(&line))
            .map(|(n, srci, dsti)| (n, srci - 1, dsti - 1))
            .for_each(|(n, srci, dsti)| {
                let src = refs[srci].take().unwrap();
                refs[dsti]
                    .as_mut()
                    .unwrap()
                    .extend(src.drain((src.len() - n)..));
                refs[srci] = Some(src);
            });
        Ok(Str(Cow::Owned(stacks_result(&stacks))))
    }
}
