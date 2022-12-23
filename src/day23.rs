use std::collections::VecDeque;

use aoc_framework::*;
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

use crate::helpers::{Direction, Point};

use Direction::*;

pub struct Day23;

impl_day!(Day23::{Part1, Part2}: 2022[23], r"
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
");

fn display(points: &FxHashSet<Point>) {
    let mut minx = isize::MAX;
    let mut maxx = isize::MIN;
    let mut miny = isize::MAX;
    let mut maxy = isize::MIN;
    points.iter().for_each(|&Point { x, y }| {
        minx = minx.min(x);
        maxx = maxx.max(x);
        miny = miny.min(y);
        maxy = maxy.max(y);
    });

    miny = miny.min(-1);
    minx = minx.min(-3);
    for y in miny..=maxy {
        for x in minx..=maxx {
            if points.contains(&Point { x, y }) {
                eprint!("#");
            } else {
                eprint!(".");
            }
        }
        eprintln!();
    }
    eprintln!();
}

fn simulate(points: &mut FxHashSet<Point>, directions: &mut VecDeque<Direction>) -> bool {
    let mut moved = false;
    let mut intents = FxHashMap::<Point, Point>::default();
    let mut counts = FxHashMap::<Point, u8>::default();
    intents.reserve(points.len());
    counts.reserve(points.len());
    for &p in points.iter() {
        let neighbors = (-1..=1)
            .flat_map(|y| (-1..=1).map(move |x| (x + p.x, y + p.y)))
            .map(|(x, y)| points.contains(&Point { x, y }))
            .collect_vec();
        if neighbors.iter().copied().filter(|&b| b).count() <= 1 {
            // no neighbors, no move
            continue;
        }
        let intent = directions
            .iter()
            .find(|&&dir| {
                ((dir as i8 - 1)..=(dir as i8 + 1))
                    .map(|d| {
                        (dir.delta() + Direction::from(d).delta()).signum() + Point { x: 1, y: 1 }
                    })
                    .all(|dest| !neighbors[dest.offset(3)])
            })
            .map(|dir| p + dir.delta());
        if let Some(dest) = intent {
            intents.insert(p, dest);
            *counts.entry(dest).or_default() += 1;
        }
    }
    for (p, dest) in intents {
        if counts[&dest] > 1 {
            continue;
        }
        points.remove(&p);
        points.insert(dest);
        moved = true;
    }
    // display(&points);
    directions.rotate_left(1);
    moved
}

fn init(input: impl Iterator<Item = String>) -> (FxHashSet<Point>, VecDeque<Direction>) {
    let mut points: FxHashSet<Point> = FxHashSet::default();
    input.enumerate().for_each(|(y, ln)| {
        ln.bytes()
            .enumerate()
            .filter(|(_, b)| *b == b'#')
            .for_each(|(x, _)| {
                points.insert((x, y).into());
            })
    });
    let mut directions = VecDeque::with_capacity(4);
    directions.push_back(Up);
    directions.push_back(Down);
    directions.push_back(Left);
    directions.push_back(Right);
    (points, directions)
}

pub struct Part1;

impl Part for Part1 {
    type D = Day23;
    const N: u8 = 1;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(110));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let (mut points, mut directions) = init(input);
        // display(&points);
        for _ in 0..10 {
            simulate(&mut points, &mut directions);
        }
        let mut minx = isize::MAX;
        let mut maxx = isize::MIN;
        let mut miny = isize::MAX;
        let mut maxy = isize::MIN;
        points.iter().for_each(|&Point { x, y }| {
            minx = minx.min(x);
            maxx = maxx.max(x);
            miny = miny.min(y);
            maxy = maxy.max(y);
        });
        let total = ((maxx - minx + 1) * (maxy - miny + 1)) as usize - points.len();
        Ok(Num(total as u64))
    }
}

pub struct Part2;

impl Part for Part2 {
    type D = Day23;
    const N: u8 = 2;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(20));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let (mut points, mut directions) = init(input);
        // display(&points);
        let res = (1..)
            .find(|_| !simulate(&mut points, &mut directions))
            .unwrap_or_default();
        Ok(Num(res))
    }
}
