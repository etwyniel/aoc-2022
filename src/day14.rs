use std::{cmp::Ordering, collections::VecDeque, ops::RangeInclusive};

use aoc_framework::*;

pub struct Day14;

impl_day!(Day14::{Part1, Part2}: 2022[14], r"
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
");

#[derive(Debug)]
enum DropResult {
    Blocked,
    Dropped { y: usize, ndx: usize },
    FellThrough,
}

use DropResult::*;

#[derive(Default, Debug, Clone)]
struct Column {
    ranges: Vec<RangeInclusive<usize>>,
}

impl Column {
    fn add_range(&mut self, range: RangeInclusive<usize>) {
        let (&l, &r) = (range.start(), range.end());
        match self.ranges.binary_search_by(|existing| {
            let (&el, &er) = (existing.start(), existing.end());
            if el > r && el - r > 1 {
                return Ordering::Greater;
            }
            if l > er && l - er > 1 {
                return Ordering::Less;
            }
            Ordering::Equal
        }) {
            Ok(i) => {
                let (&el, &er) = (self.ranges[i].start(), self.ranges[i].end());
                self.ranges[i] = (el.min(l))..=(er.max(r));
            }
            Err(i) => self.ranges.insert(i, range),
        }
    }

    fn drop(&self, y: usize) -> DropResult {
        match self.ranges.binary_search_by(|range| {
            if y < *range.start() {
                return Ordering::Greater;
            }
            if y > *range.end() {
                return Ordering::Less;
            }
            Ordering::Equal
        }) {
            Ok(_) => Blocked,
            Err(i) => {
                if i == self.ranges.len() {
                    FellThrough
                } else {
                    let next_y = self.ranges[i].start() - 1;
                    Dropped { y: next_y, ndx: i }
                }
            }
        }
    }

    fn add_grain(&mut self, ndx: usize, y: usize) {
        let current = &self.ranges[ndx];
        if ndx > 0 {
            let prev = &self.ranges[ndx - 1];
            if current.start() - prev.end() <= 2 {
                self.ranges[ndx - 1] = *prev.start()..=*current.end();
                self.ranges.remove(ndx);
                return;
            }
        }
        self.ranges[ndx] = y..=*current.end();
    }
}

#[derive(Debug, Clone)]
struct Step {
    x: usize,
    y: usize,
    ndx: usize,
}

struct Cave {
    offset: usize,
    columns: VecDeque<Option<Column>>,
    steps: Vec<Step>,
}

impl Default for Cave {
    fn default() -> Self {
        let mut columns = VecDeque::new();
        columns.push_back(None);
        Cave {
            offset: 500,
            columns,
            steps: vec![Step {
                x: 500,
                y: 0,
                ndx: 0,
            }],
        }
    }
}

impl Cave {
    fn add_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let low_x = x1.min(x2);
        let high_x = x1.max(x2);
        if low_x < self.offset {
            for _ in low_x..self.offset {
                self.columns.push_front(None);
            }
            self.offset = low_x;
        }
        if high_x - self.offset > self.columns.len() {
            for _ in (self.offset + self.columns.len())..=high_x {
                self.columns.push_back(None);
            }
        }
        if x1 == x2 {
            self.columns[x1 - self.offset]
                .get_or_insert_with(Column::default)
                .add_range((y1.min(y2))..=(y1.max(y2)));
        } else {
            for x in low_x..=high_x {
                self.columns[x - self.offset]
                    .get_or_insert_with(Column::default)
                    .add_range(y1..=y1);
            }
        }
    }

    fn drop(&self, x: usize, y: usize) -> DropResult {
        if x < self.offset || x - self.offset >= self.columns.len() {
            return FellThrough;
        }
        match &self.columns[x - self.offset] {
            None => FellThrough,
            Some(col) => col.drop(y),
        }
    }

    fn step(&mut self) -> bool {
        // dbg!(&self.steps);
        let mut step = loop {
            let Some(last) = self.steps.last() else {
            return false;
        };
            match self.drop(last.x, last.y) {
                FellThrough => return true,
                Blocked => {
                    self.steps.pop();
                }
                Dropped { y, ndx } => {
                    let step = Step { x: last.x, y, ndx };
                    self.steps.push(step.clone());
                    break step;
                }
            }
        };
        'outer: loop {
            for dx in [-1, 1] {
                let x = (step.x as isize + dx) as usize;
                match self.drop(x, step.y + 1) {
                    FellThrough => return false,
                    Dropped { y, ndx } => {
                        step.x = x;
                        step.y = y;
                        step.ndx = ndx;
                        self.steps.push(step.clone());
                        continue 'outer;
                    }
                    Blocked => {}
                }
            }
            self.columns[step.x - self.offset]
                .as_mut()
                .unwrap()
                .add_grain(step.ndx, step.y);
            return true;
        }
    }
}

pub struct Part1;

impl Part for Part1 {
    type D = Day14;
    const N: u8 = 1;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(24));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let mut cave = Cave::default();
        for line in input {
            line.split(" -> ")
                .map(|point| {
                    let (x, y) = point.split_once(',').unwrap();
                    (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
                })
                .tuple_windows()
                .for_each(|((x1, y1), (x2, y2))| cave.add_line(x1, y1, x2, y2));
        }
        let mut step = 0;
        while cave.step() {
            step += 1;
        }
        Ok(Num(step))
    }
}

pub struct Part2;

impl Part for Part2 {
    type D = Day14;
    const N: u8 = 2;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(93));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let mut highest_y = 0;
        let mut lowest_x = usize::MAX;
        let mut highest_x = 0;
        let mut cave = Cave::default();
        for line in input {
            line.split(" -> ")
                .map(|point| {
                    let (x, y) = point.split_once(',').unwrap();
                    (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
                })
                .tuple_windows()
                .for_each(|((x1, y1), (x2, y2))| {
                    highest_y = highest_y.max(y1).max(y2);
                    lowest_x = lowest_x.min(x1).min(x2);
                    highest_x = highest_x.max(x1).max(x2);
                    cave.add_line(x1, y1, x2, y2);
                });
        }
        let floor_y = highest_y + 2;
        cave.add_line(lowest_x - floor_y, floor_y, highest_x + floor_y, floor_y);
        let mut step = 0;
        while cave.step() {
            step += 1;
        }
        Ok(Num(step))
    }
}
