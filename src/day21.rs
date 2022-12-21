use aoc_framework::*;

use aoc_framework::anyhow::bail;

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

use std::fmt::{self, Debug, Formatter, Write};
use std::str::FromStr;

pub struct Day21;

impl_day!(Day21::{Part1, Part2}: 2022[21], r"
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
");

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Id(u32);

impl Debug for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        for i in 0..4 {
            let c = 'a' as u32 + (self.0 >> (8 * (3 - i))) & 0xff;
            f.write_char(char::from_u32(c).unwrap())?;
        }
        Ok(())
    }
}

impl FromStr for Id {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Id> {
        Ok(Id(s
            .bytes()
            .take(4)
            .map(|b| (b - b'a') as u32)
            .enumerate()
            .fold(0, |acc, (i, b)| acc | (b << (8 * (3 - i))))))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Operation {
    Add,
    Sub,
    Div,
    Mul,
}

use Operation::*;

#[derive(Debug, Clone)]
enum Node {
    Value(i64),
    Op(Operation, [Id; 2]),
    Human,
}

use Node::*;

impl FromStr for Node {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Node> {
        if let Ok(n) = s.parse() {
            return Ok(Value(n));
        }
        let Some((lhs, op, rhs)) = s.split(' ').tuples().next() else {
            bail!("Invalid node");
        };
        let operands = [lhs.parse()?, rhs.parse()?];
        let op = match op {
            "+" => Add,
            "-" => Sub,
            "/" => Div,
            "*" => Mul,
            _ => bail!("Invalid operation {op}"),
        };
        Ok(Op(op, operands))
    }
}

fn parse_expressions(input: impl Iterator<Item = String>) -> anyhow::Result<FxHashMap<Id, Node>> {
    let mut out = FxHashMap::default();
    for line in input {
        let Some((id, op)) = line.split_once(": ") else {
            bail!("Invalid expression: {line}");
        };
        out.insert(id.parse()?, op.parse()?);
    }
    Ok(out)
}

pub struct Part1;

impl Part for Part1 {
    type D = Day21;
    const N: u8 = 1;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(152));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let mut exprs = parse_expressions(input)?;
        let root = "root".parse().unwrap();
        let mut stack: Vec<Id> = vec![root];
        while let Some(id) = stack.last().copied() {
            let Op(op, operands) = exprs[&id] else {
                stack.pop();
                continue;
            };
            let Some((lhs, rhs)) = operands.iter().flat_map(|id| {
                match exprs[id] {
                    Value(v) => Some(v),
                    _ => {stack.push(*id); None}
                }
            }).tuples().next() else {
                continue;
            };
            stack.pop();
            let val = match op {
                Add => lhs + rhs,
                Sub => lhs - rhs,
                Mul => lhs * rhs,
                Div => lhs / rhs,
            };
            exprs.insert(id, Value(val));
        }
        let Value(rootval) = exprs[&root] else {
            bail!("Could not compute value of root");
        };
        Ok(Num(rootval as u64))
    }
}

type Expressions = FxHashMap<Id, Node>;

fn eval_pt2(exprs: &Expressions, new_exprs: &Expressions, root: Id, humn_val: i64) -> anyhow::Result<i64> {
    let mut new_exprs = new_exprs.clone();
    let humn = "humn".parse().unwrap();
    new_exprs.insert(humn, Value(humn_val));
    let mut stack: Vec<Id> = vec![root];
    while let Some(id) = stack.last().copied() {
        let Op(op, operands) = new_exprs.get(&id).or_else(|| exprs.get(&id)).unwrap() else {
            stack.pop();
            continue;
        };
        let Some((lhs, rhs)) = operands.iter().flat_map(|id| {
            match new_exprs.get(id).or_else(|| exprs.get(id)).unwrap() {
                Value(v) => Some(v),
                _ => {stack.push(*id); None}
            }
        }).tuples().next() else {
            continue;
        };
        stack.pop();
        let val = match op {
            Add => lhs + rhs,
            Sub => lhs - rhs,
            Mul => lhs * rhs,
            Div => lhs / rhs,
        };
        new_exprs.insert(id, Value(val));
    }
    let Value(rootval) = new_exprs[&root] else {
        bail!("Could not compute value of root");
    };
    Ok(rootval)
}

pub struct Part2;

impl Part for Part2 {
    type D = Day21;
    const N: u8 = 2;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(301));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let mut exprs = parse_expressions(input)?;
        let root = "root".parse().unwrap();
        let humn = "humn".parse().unwrap();
        exprs.insert(humn, Human);
        let mut stack: Vec<Id> = vec![root];
        let mut depend_on_humn = FxHashSet::default();
        while let Some(id) = stack.last().copied() {
            let Op(op, operands) = exprs[&id] else {
                stack.pop();
                continue;
            };
            let Some((lhs, rhs)) = operands.iter().enumerate().flat_map(|(i, op_id)| {
                if depend_on_humn.contains(op_id) {
                    depend_on_humn.insert(id);
                    for _ in 0..(i+1) {
                    stack.pop();
                    }
                    return None;
                }
                match exprs[op_id] {
                    Value(v) => Some(v),
                    Human => {
                        depend_on_humn.insert(id);
                    for _ in 0..(i+1) {
                    stack.pop();
                    }
                        None
                    }
                    _ => {stack.push(*op_id); None}
                }
            }).tuples().next() else {
                continue;
            };
            stack.pop();
            let val = match op {
                Add => lhs + rhs,
                Sub => lhs - rhs,
                Mul => lhs * rhs,
                Div => lhs / rhs,
            };
            exprs.insert(id, Value(val));
        }
        let Op(_, [lhs, rhs]) = exprs[&root] else {
            panic!()
        };
        let (new_root, &target) = match (&exprs[&lhs], &exprs[&rhs]) {
            (Value(v), Op(_, _)) => (rhs, v),
            (Op(_, _), Value(v)) => (lhs, v),
            _ => bail!("Could not find target value"),
        };
        let new_exprs: FxHashMap<Id, Node> = exprs
            .iter()
            .filter(|(id, _)| depend_on_humn.contains(id))
            .map(|(&id, node)| (id, node.clone()))
            .collect();
        let eval = |v| eval_pt2(&exprs, &new_exprs, new_root, v);
        let growing = eval(1000)? < eval(2000)?;
        let mut range = 0..1_000_000_000_000_000;
        loop {
            let mut humn_val = range.start + (range.end - range.start) / 2;
            let root_val = eval(humn_val)?;
            if (root_val < target && growing) || (target < root_val && !growing) {
                // eprintln!("{root_val} != {target} ({humn_val})");
                range = (humn_val + 1)..range.end;
            } else if (root_val > target && growing) || (target > root_val && !growing) {
                // eprintln!("{root_val} != {target} ({humn_val})");
                range = range.start..humn_val;
            } else {
                while eval(humn_val - 1)? == target {
                    humn_val -= 1;
                }
                return Ok(Num(humn_val as u64));
            }
        }
        bail!("Could not compute value of root");
        Ok(Num(0))
    }
}
