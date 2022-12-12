use std::{cmp::Reverse, collections::VecDeque};

use aoc_framework::{
    anyhow::{anyhow, Context},
    *,
};

pub struct Day11;

impl_day!(Day11::{Part1, Part2}: 2022[11], r"
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
");

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Operand {
    Old,
    Const(u64),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Operation {
    Add,
    Mul,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    operand: Operand,
    div_test: u64,
    target_true: usize,
    target_false: usize,

    inspected: u64,
}

impl Monkey {
    fn run_op(&self, item: u64) -> u64 {
        let value = match self.operand {
            Operand::Old => item,
            Operand::Const(n) => n,
        };
        match self.operation {
            Operation::Add => item + value,
            Operation::Mul => item * value,
        }
    }

    fn run_turn(&mut self) -> Option<(u64, usize)> {
        let item = self.items.pop_front()?;
        self.inspected += 1;
        let new_item = self.run_op(item) / 3;
        let target = if new_item % self.div_test == 0 {
            self.target_true
        } else {
            self.target_false
        };
        Some((new_item, target))
    }

    fn run_turn_2(&mut self, divisor: u64) -> Option<(u64, usize)> {
        let item = self.items.pop_front()?;
        self.inspected += 1;
        let new_item = self.run_op(item) % divisor;
        let target = if new_item % self.div_test == 0 {
            self.target_true
        } else {
            self.target_false
        };
        Some((new_item, target))
    }
}

fn parse_monkey(mut input: impl Iterator<Item = String>) -> anyhow::Result<Monkey> {
    input.next();
    let items: VecDeque<u64> = input
        .next()
        .ok_or_else(|| anyhow!("Invalid input"))?
        .split_once(": ")
        .ok_or_else(|| anyhow!("Invalid input"))?
        .1
        .split(", ")
        .map(|item| item.parse().context("Invalid input"))
        .collect::<anyhow::Result<_>>()?;
    let (operation, operand) = input
        .next()
        .and_then(|line| {
            let (op, operand) = line.split_once("= old ")?.1.split_once(' ')?;
            let op = match op {
                "*" => Operation::Mul,
                "+" => Operation::Add,
                _ => return None,
            };
            let operand = match operand {
                "old" => Operand::Old,
                _ => Operand::Const(operand.parse().ok()?),
            };
            Some((op, operand))
        })
        .ok_or_else(|| anyhow!("Invalid input"))?;
    let div_test: u64 = input
        .next()
        .and_then(|line| line.rsplit_once(' ')?.1.parse().ok())
        .ok_or_else(|| anyhow!("Invalid input"))?;
    let target_true: usize = input
        .next()
        .and_then(|line| line.rsplit_once(' ')?.1.parse().ok())
        .ok_or_else(|| anyhow!("Invalid input"))?;
    let target_false: usize = input
        .next()
        .and_then(|line| line.rsplit_once(' ')?.1.parse().ok())
        .ok_or_else(|| anyhow!("Invalid input"))?;
    input.next(); // empty line
    Ok(Monkey {
        items,
        operation,
        operand,
        div_test,
        target_true,
        target_false,
        inspected: 0,
    })
}

pub struct Part1;

impl Part for Part1 {
    type D = Day11;
    const N: u8 = 1;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(10605));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let mut input = input.peekable();
        let mut monkeys: Vec<Monkey> = Vec::new();
        while input.peek().is_some() {
            let monkey = parse_monkey(&mut input)?;
            monkeys.push(monkey);
        }
        for _ in 0..20 {
            for i in 0..monkeys.len() {
                while let Some((item, target)) = monkeys[i].run_turn() {
                    monkeys[target].items.push_back(item);
                }
            }
        }
        let mut counts = monkeys
            .iter()
            .map(|m| Reverse(m.inspected))
            .collect::<Vec<_>>();
        counts.sort_unstable();
        Ok(Num(counts[0].0 * counts[1].0))
    }
}

pub struct Part2;

impl Part for Part2 {
    type D = Day11;
    const N: u8 = 2;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(2713310158));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let mut input = input.peekable();
        let mut monkeys: Vec<Monkey> = Vec::new();
        let mut common_factor = 1;
        while input.peek().is_some() {
            let monkey = parse_monkey(&mut input)?;
            common_factor *= monkey.div_test;
            monkeys.push(monkey);
        }
        for _ in 0..10_000 {
            for i in 0..monkeys.len() {
                while let Some((item, target)) = monkeys[i].run_turn_2(common_factor) {
                    monkeys[target].items.push_back(item);
                }
            }
        }
        let mut counts = monkeys
            .iter()
            .map(|m| Reverse(m.inspected))
            .collect::<Vec<_>>();
        counts.sort_unstable();
        Ok(Num(counts[0].0 * counts[1].0))
    }
}
