use std::str::FromStr;

use aoc_framework::{
    anyhow::{anyhow, bail, ensure},
    *,
};

pub struct Day7;

impl_day!(Day7::{Part1, Part2}: 2022[7], r"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
");

#[derive(Debug)]
enum Entry {
    Dir { name: String, entries: Vec<Entry> },
    File { size: u64 },
}

use Entry::*;

impl Entry {
    fn entries_mut(&mut self) -> Option<&mut Vec<Entry>> {
        if let Dir { entries, .. } = self {
            Some(entries)
        } else {
            None
        }
    }

    fn size(&self) -> u64 {
        match self {
            Dir { entries, .. } => entries.iter().map(|entry| entry.size()).sum(),
            File { size, .. } => *size,
        }
    }

    fn size_under_100k(&self, sum: &mut u64) -> u64 {
        match self {
            Dir { entries, .. } => {
                let size: u64 = entries.iter().map(|entry| entry.size_under_100k(sum)).sum();
                if size < 100_000 {
                    *sum += size;
                }
                size
            }
            File { size, .. } => *size,
        }
    }

    fn size_smallest_over(&self, val: u64, current: &mut Option<u64>) -> u64 {
        match self {
            Dir { entries, .. } => {
                let size: u64 = entries
                    .iter()
                    .map(|entry| entry.size_smallest_over(val, current))
                    .sum();
                if size >= val {
                    let curr = current.unwrap_or(u64::MAX);
                    if size < curr {
                        *current = Some(size);
                    }
                }
                size
            }
            File { size, .. } => *size,
        }
    }
}

impl FromStr for Entry {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(name) = s.strip_prefix("dir ") {
            return Ok(Dir {
                name: name.to_string(),
                entries: Vec::new(),
            });
        }
        let Some((size, _)) = s.split_once(' ') else {
            bail!("Invalid input");
        };
        Ok(File {
            size: size.parse()?,
        })
    }
}

fn current_entry(stack: &mut [Entry]) -> anyhow::Result<&mut Vec<Entry>> {
    stack
        .last_mut()
        .map(|e| e.entries_mut().unwrap())
        .ok_or_else(|| anyhow!("Invalid input: reached root"))
}

fn pop(stack: &mut Vec<Entry>) -> anyhow::Result<&mut Vec<Entry>> {
    let entry = stack.pop().unwrap();
    let current = current_entry(stack)?;
    current.push(entry);
    Ok(current)
}

fn build_fs(mut input: impl Iterator<Item = String>) -> anyhow::Result<Entry> {
    ensure!(input.next().as_deref() == Some("$ cd /"));
    let mut stack = vec![Dir {
        name: "/".to_string(),
        entries: Vec::new(),
    }];
    let mut input = input.peekable();
    let mut current = current_entry(&mut stack)?;
    while let Some(cmd) = input.next() {
        if let Some(dir) = cmd.strip_prefix("$ cd ") {
            if dir == ".." {
                current = pop(&mut stack)?;
                continue;
            }
            let ndx = current
                .iter()
                .position(|entry| {
                    if let Dir { name, .. } = entry {
                        name == dir
                    } else {
                        false
                    }
                })
                .ok_or_else(|| anyhow!("Invalid input: directory {dir} not found"))?;
            let entry = current.remove(ndx);
            stack.push(entry);
            current = current_entry(&mut stack)?;
        } else if cmd == "$ ls" {
            while input.peek().map(|l| !l.starts_with('$')) == Some(true) {
                let line = input.next().unwrap();
                current.push(line.parse()?);
            }
        } else {
            bail!("Invalid input: could not process line {cmd}")
        }
    }
    while stack.len() > 1 {
        pop(&mut stack)?;
    }
    Ok(stack.pop().unwrap())
}

pub struct Part1;

impl Part for Part1 {
    type D = Day7;
    const N: u8 = 1;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(95437));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let root = build_fs(input)?;
        let mut sum = 0;
        root.size_under_100k(&mut sum);
        Ok(Num(sum))
    }
}

pub struct Part2;

impl Part for Part2 {
    type D = Day7;
    const N: u8 = 2;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(24933642));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let root = build_fs(input)?;
        let total_used = root.size();
        let required = 30_000_000 - (70000000 - total_used);
        let mut size_deleted = None;
        root.size_smallest_over(required, &mut size_deleted);
        Ok(Num(size_deleted.unwrap()))
    }
}
