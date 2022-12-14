use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, HashMap},
    ops::RangeInclusive,
};

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
    fn can_move(&self, y: usize) -> bool {
        for range in &self.ranges {
            if *range.start() > y {
                return true;
            }
            if range.contains(&y) {
                return false;
            }
        }
        true
    }

    fn add_range(&mut self, range: RangeInclusive<usize>) {
        let (&l, &r) = (range.start(), range.end());
        // for (i, existing) in self.ranges.iter_mut().enumerate() {
        //     let (&el, &er) = (existing.start(), existing.end());
        //     if el > r {
        //         if el - r == 1 {
        //             *existing = l..=er;
        //             return;
        //         }
        //         self.ranges.insert(i, range);
        //         return;
        //     }
        //     if l > er {
        //         if l - r == 1 {
        //             *existing = el..=r;
        //             return;
        //         }
        //         continue;
        //     }
        //     *existing = (el.min(l))..=(er.max(r));
        //     return;
        // }
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

#[derive(Default)]
struct Cave(HashMap<usize, Column>);

impl Cave {
    fn add_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        if x1 == x2 {
            self.0
                .entry(x1)
                .or_default()
                .add_range((y1.min(y2))..=(y1.max(y2)));
        } else {
            let (xl, xr) = (x1.min(x2), x1.max(x2));
            for x in xl..=xr {
                self.0.entry(x).or_default().add_range(y1..=y1);
            }
        }
    }

    fn drop(&self, x: usize, y: usize) -> DropResult {
        let Some(column) = self.0.get(&x) else {
            return FellThrough;
        };
        column.drop(y)
    }

    fn step(&mut self) -> bool {
        let mut cur_x = 500;
        let mut cur_y = 0;
        let mut cur_ndx;
        match self.drop(cur_x, cur_y) {
            FellThrough | Blocked => return false,
            Dropped { y, ndx } => {
                cur_y = y;
                cur_ndx = ndx;
            }
        }
        loop {
            match self.drop(cur_x - 1, cur_y + 1) {
                FellThrough => return false,
                Dropped { y, ndx } => {
                    cur_x -= 1;
                    cur_y = y;
                    cur_ndx = ndx;
                    continue;
                }
                Blocked => {}
            }
            match self.drop(cur_x + 1, cur_y + 1) {
                FellThrough => return false,
                Dropped { y, ndx } => {
                    cur_x += 1;
                    cur_y = y;
                    cur_ndx = ndx;
                    continue;
                }
                Blocked => {}
            }
            self.0.get_mut(&cur_x).unwrap().add_grain(cur_ndx, cur_y);
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
