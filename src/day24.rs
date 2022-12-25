use std::{cmp::Reverse, collections::BinaryHeap, iter::once};

use aoc_framework::*;
use fxhash::FxHashMap;

use aoc_framework::anyhow::bail;

use crate::helpers::{Direction, Point};

use Direction::*;

pub struct Day24;

impl_day!(Day24::{Part1, Part2}: 2022[24], r"
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
");

struct Blizzard {
    starting_pos: Point,
    dir: Direction,
}

struct Grid {
    blizzards: Vec<Blizzard>,
    w: usize,
    h: usize,
    start: Point,
    end: Point,
}

fn parse(mut input: impl Iterator<Item = String>) -> Grid {
    let first = input.next().unwrap();
    let w = first.len() - 2;
    let start_x = first.find('.').unwrap() - 1;
    let mut end_x = 0;
    let mut h = 1;
    let mut blizzards = Vec::new();
    for ln in input {
        end_x = ln.find('.').unwrap_or(1) - 1;
        for (x, b) in ln
            .bytes()
            .skip(1)
            .enumerate()
            .filter(|(_, b)| ![b'.', b'#'].contains(b))
        {
            let dir = match b {
                b'>' => Right,
                b'v' => Down,
                b'<' => Left,
                b'^' => Up,
                _ => panic!("Invalid input"),
            };
            blizzards.push(Blizzard {
                starting_pos: Point {
                    x: x as isize,
                    y: h as isize,
                },
                dir,
            });
        }
        h += 1;
    }
    Grid {
        blizzards,
        w,
        h,
        start: Point {
            x: start_x as isize,
            y: 0,
        },
        end: Point {
            x: end_x as isize,
            y: h as isize - 1,
        },
    }
}

impl Grid {
    fn at_step(&self, step: usize) -> Vec<bool> {
        let mut out_grid = vec![false; self.w * self.h];
        let h = self.h as isize - 2;
        let w = self.w as isize;
        for x in 0..self.w {
            let p1 = Point::from((x, 0));
            let p2 = Point::from((x, self.h - 1));
            out_grid[p1.offset(self.w)] = true;
            out_grid[p2.offset(self.w)] = true;
        }
        out_grid[self.start.offset(self.w)] = false;
        out_grid[self.end.offset(self.w)] = false;

        for b in &self.blizzards {
            let delta = b.dir.delta() * step as isize;
            let mut p = b.starting_pos + delta;
            p.x = p.x.rem_euclid(w);
            p.y = (p.y - 1).rem_euclid(h) + 1;
            out_grid[p.offset(self.w)] = true;
        }

        out_grid
    }

    fn disp_at_step(&self, step: usize) {
        let mask = self.at_step(step);
        for y in 0..self.h {
            for x in 0..self.w {
                if mask[y * self.w + x] {
                    eprint!("#");
                } else {
                    eprint!(".");
                }
            }
            eprintln!();
        }
        eprintln!();
    }

    fn find_path(&self, start_step: u64, reversed: bool) -> anyhow::Result<u64> {
        let mut set = BinaryHeap::new();
        let start_step = Step {
            step: start_step as usize,
            pos: if reversed { self.end } else { self.start },
        };
        set.push(Reverse(Entry {
            step: start_step,
            cost: 0,
        }));
        let target = if reversed { self.start } else { self.end };

        let mut g_score = FxHashMap::default();
        g_score.insert(start_step, 0);

        while !set.is_empty() {
            let Reverse(current) = set.pop().unwrap();
            let Step { step, pos } = current.step;
            if current.step.pos == target {
                return Ok(step as u64);
            }
            let mask = self.at_step(step + 1);
            for delta in [(0isize, 0isize), (-1, 0), (1, 0), (0, -1), (0, 1)] {
                let neigh = pos + Point::from(delta);
                if neigh.is_oob(self.w, self.h) || mask[neigh.offset(self.w)] {
                    // blocked next step
                    continue;
                }
                let neigh_step = Step {
                    step: step + 1,
                    pos: neigh,
                };
                let tentative_g_score = g_score[&current.step] + 1;
                if g_score
                    .get(&neigh_step)
                    .map(|&cost| tentative_g_score < cost)
                    .unwrap_or(true)
                {
                    g_score.insert(neigh_step, tentative_g_score);
                    if !set.iter().any(|existing| existing.0.step == neigh_step) {
                        let neigh_entry = Entry {
                            step: neigh_step,
                            cost: tentative_g_score,
                        };
                        set.push(Reverse(neigh_entry));
                    }
                }
            }
        }
        bail!("Could not find a path");
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Step {
    pos: Point,
    step: usize,
}

struct Entry {
    step: Step,
    cost: u64,
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Entry {}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost
            .cmp(&other.cost)
            .then(self.step.step.cmp(&other.step.step))
    }
}

pub struct Part1;

impl Part for Part1 {
    type D = Day24;
    const N: u8 = 1;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(18));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let grid = parse(input);
        // for step in 0..5 {
        //     grid.disp_at_step(step);
        // }
        Ok(Num(grid.find_path(0, false)?))
    }
}

pub struct Part2;

impl Part for Part2 {
    type D = Day24;
    const N: u8 = 2;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(54));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let grid = parse(input);
        let step = grid.find_path(0, false)?;
        let step = grid.find_path(step, true)?;
        let step = grid.find_path(step, false)?;
        Ok(Num(step))
    }
}
