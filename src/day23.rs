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

struct State {
    points: FxHashSet<Point>,
    directions: VecDeque<Direction>,
    intents: Vec<(Point, Point)>,
    counts: FxHashMap<Point, u8>,
}

impl State {
    fn simulate(&mut self) -> bool {
        let mut moved = false;
        self.intents.clear();
        self.counts.clear();
        // Avoid iterating on VecDeque in hot loop
        let directions: [Direction; 4] = [
            self.directions[0],
            self.directions[1],
            self.directions[2],
            self.directions[3],
        ];
        for &p in self.points.iter() {
            let mut neighbors = [false; 3 * 3];
            let mut count = 0;
            neighbors.iter_mut().enumerate().for_each(|(i, b)| {
                *b = i != 4
                    && self.points.contains(&Point {
                        x: p.x + (i % 3 - 1) as isize,
                        y: p.y + (i / 3 - 1) as isize,
                    });
                if *b {
                    count += 1;
                }
            });
            if count == 0 {
                // no neighbors, no move
                continue;
            }
            let intent = directions
                .into_iter()
                .find(|&dir| {
                    ((dir as i8 - 1)..=(dir as i8 + 1))
                        .map(|d| (dir.delta() | Direction::from(d).delta()) + Point { x: 1, y: 1 })
                        .all(|dest| !neighbors[dest.offset(3)])
                })
                .map(|dir| p + dir.delta());
            if let Some(dest) = intent {
                self.intents.push((p, dest));
                *self.counts.entry(dest).or_default() += 1;
            }
        }
        for &(p, dest) in &self.intents {
            if self.counts[&dest] > 1 {
                continue;
            }
            self.points.remove(&p);
            self.points.insert(dest);
            moved = true;
        }
        // self.display();
        self.directions.rotate_left(1);
        moved
    }

    fn new(input: impl Iterator<Item = String>) -> Self {
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
        let intents = Vec::with_capacity(points.len());
        let mut counts = FxHashMap::default();
        counts.reserve(points.len());
        State {
            points,
            directions,
            intents,
            counts,
        }
    }

    fn display(&self) {
        let mut minx = isize::MAX;
        let mut maxx = isize::MIN;
        let mut miny = isize::MAX;
        let mut maxy = isize::MIN;
        self.points.iter().for_each(|&Point { x, y }| {
            minx = minx.min(x);
            maxx = maxx.max(x);
            miny = miny.min(y);
            maxy = maxy.max(y);
        });

        miny = miny.min(-1);
        minx = minx.min(-3);
        for y in miny..=maxy {
            for x in minx..=maxx {
                if self.points.contains(&Point { x, y }) {
                    eprint!("#");
                } else {
                    eprint!(".");
                }
            }
            eprintln!();
        }
        eprintln!();
    }
}

pub struct Part1;

impl Part for Part1 {
    type D = Day23;
    const N: u8 = 1;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(110));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let mut state = State::new(input);
        // display(&points);
        for _ in 0..10 {
            state.simulate();
        }
        let mut minx = isize::MAX;
        let mut maxx = isize::MIN;
        let mut miny = isize::MAX;
        let mut maxy = isize::MIN;
        state.points.iter().for_each(|&Point { x, y }| {
            minx = minx.min(x);
            maxx = maxx.max(x);
            miny = miny.min(y);
            maxy = maxy.max(y);
        });
        let total = ((maxx - minx + 1) * (maxy - miny + 1)) as usize - state.points.len();
        Ok(Num(total as u64))
    }
}

pub struct Part2;

impl Part for Part2 {
    type D = Day23;
    const N: u8 = 2;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(20));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let mut state = State::new(input);
        // display(&points);
        let res = (1..).find(|_| !state.simulate()).unwrap_or_default();
        Ok(Num(res))
    }
}
