use std::collections::VecDeque;

use aoc_framework::*;

pub struct Day17;

impl_day!(Day17::{Part1, Part2}: 2022[17], r">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");

#[derive(Copy, Clone)]
struct Line(u8);

const SHAPES: [&[Line]; 5] = [
    &[
        Line(0b1111000), // ####
    ],
    &[
        Line(0b0100000), // .#.
        Line(0b1110000), // ###
        Line(0b0100000), // .#.
    ],
    &[
        Line(0b1110000), // ..#
        Line(0b0010000), // ..#
        Line(0b0010000), // ###
    ],
    &[
        Line(0b1000000), // #
        Line(0b1000000), // #
        Line(0b1000000), // #
        Line(0b1000000), // #
    ],
    &[
        Line(0b1100000), // ##
        Line(0b1100000), // ##
    ],
];

const TOTAL_STEPS: usize = 2022;
const STEPS_PT2: usize = 1_000_000_000_000;

#[derive(Debug, PartialEq, Eq)]
enum Dir {
    Left,
    Right,
}

fn clashes(lines: &VecDeque<Line>, pos: usize, shape: &[Line], offset: usize) -> bool {
    for (i, ln) in shape.iter().enumerate() {
        let shifted = ln.0 >> offset;
        if shifted.count_ones() != ln.0.count_ones() {
            return true;
        }
        if lines[pos + i].0 & shifted != 0 {
            return true;
        }
    }
    false
}

fn print_lines(lines: &VecDeque<Line>) {
    for Line(ln) in lines.iter().rev() {
        for off in 0..7 {
            if (ln >> (6 - off)) & 1 == 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn print_shape(shape: &VecDeque<Line>, offset: usize) {
    for Line(ln) in shape.iter().rev() {
        for off in 0..7 {
            if ((ln >> offset) >> (6 - off)) & 1 == 1 {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn trim_lines(lines: &mut VecDeque<Line>) -> usize {
    if lines.back().map(|&Line(ln)| ln != 0).unwrap_or(true) {
        lines.push_back(Line(0));
    }
    let mut reachable: Vec<u8> = vec![0; lines.len()];
    *reachable.last_mut().unwrap() = 0b1111111;
    for (i, Line(ln)) in lines.iter().enumerate().rev().skip(1) {
        for off in 0usize..7 {
            let reachable_up = (reachable[i + 1] >> off) & 1 == 1;
            let reachable_right = (reachable[i] >> off.saturating_sub(1)) & 1 == 1;
            if (ln >> off) & 1 == 0 && (reachable_up || reachable_right) {
                reachable[i] |= 1 << off;
            }
        }
        for off in 0usize..7 {
            let reachable_left = (reachable[i] >> (6 - off + 1)) & 1 == 1;
            if (ln >> (6 - off)) & 1 == 0 && reachable_left {
                reachable[i] |= 1 << (6 - off);
            }
        }
        if reachable[i] == 0 {
            break;
        }
    }
    // for (&ln, &Line(src)) in reachable.iter().zip(lines.iter()).rev() {
    //     for off in 0..7 {
    //         let r = (ln >> (6 - off)) & 1 == 1;
    //         if (src >> (6 - off)) & 1 == 1 {
    //             if r {
    //                 print!("!");
    //             } else {
    //                 print!("#");
    //             }
    //         } else if r {
    //             print!(" ");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }
    let mut trimmed = 0;
    reachable
        .into_iter()
        .take_while(|&line| line == 0)
        .for_each(|_| {
            trimmed += 1;
            lines.pop_front();
        });
    trimmed
}

pub struct Part1;

impl Part for Part1 {
    type D = Day17;
    const N: u8 = 1;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(3068));

    fn run(mut input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let directions: Vec<Dir> = input
            .next()
            .unwrap_or_default()
            .trim()
            .bytes()
            .map(|b| if b == b'>' { Dir::Right } else { Dir::Left })
            .collect();
        let mut lines: VecDeque<Line> = VecDeque::new();
        let mut dir_ndx = 0;
        let mut bottom = 0;
        for n in 0..TOTAL_STEPS {
            let mut offset: usize = 2;
            let shape = SHAPES[n % SHAPES.len()];
            // print_shape(shape, offset);
            let mut n_empty = lines.iter().rev().take_while(|ln| ln.0 == 0).count();
            let top = n_empty;
            for _ in top..(shape.len() + 3) {
                lines.push_back(Line(0));
                n_empty += 1;
            }
            let mut pos = (lines.len() - n_empty) + 3;
            loop {
                let next_offset = if directions[dir_ndx] == Dir::Left {
                    offset.saturating_sub(1)
                } else {
                    offset + 1
                };
                dir_ndx = (dir_ndx + 1) % directions.len();
                if !clashes(&lines, pos, shape, next_offset) {
                    offset = next_offset;
                }
                if pos == 0 || clashes(&lines, pos - 1, shape, offset) {
                    for (i, Line(ln)) in shape.iter().enumerate() {
                        lines[pos + i] = Line(lines[pos + i].0 | ln >> offset);
                    }
                    break;
                }
                pos -= 1;
            }
            // print_lines(&lines);
            let trimmed = trim_lines(&mut lines);
            bottom += trimmed;
        }
        let n_empty = lines.iter().rev().take_while(|ln| ln.0 == 0).count();
        Ok(Num((lines.len() - n_empty + bottom) as u64))
    }
}

pub struct Part2;

impl Part for Part2 {
    type D = Day17;
    const N: u8 = 2;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(1514285714288));

    fn run(mut input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let directions: Vec<Dir> = input
            .next()
            .unwrap_or_default()
            .trim()
            .bytes()
            .map(|b| if b == b'>' { Dir::Right } else { Dir::Left })
            .collect();
        let mut lines: VecDeque<Line> = VecDeque::new();
        let mut dir_ndx = 0;
        let mut bottom = 0;
        // let mut memo = vec![None; directions.len()];
        let mut n = 0;
        let mut prev = Vec::new();
        let mut skipped = false;
        while n < STEPS_PT2 {
            let mut offset: usize = 2;
            let shape = SHAPES[n % SHAPES.len()];
            let mut n_empty = lines.iter().rev().take_while(|ln| ln.0 == 0).count();
            let top = n_empty;
            for _ in top..(shape.len() + 3) {
                lines.push_back(Line(0));
                n_empty += 1;
            }
            let mut pos = (lines.len() - n_empty) + 3;
            loop {
                let next_offset = if directions[dir_ndx] == Dir::Left {
                    offset.saturating_sub(1)
                } else {
                    offset + 1
                };
                dir_ndx = (dir_ndx + 1) % directions.len();
                if !clashes(&lines, pos, shape, next_offset) {
                    offset = next_offset;
                }
                if pos == 0 || clashes(&lines, pos - 1, shape, offset) {
                    for (i, Line(ln)) in shape.iter().enumerate() {
                        lines[pos + i] = Line(lines[pos + i].0 | ln >> offset);
                    }
                    break;
                }
                pos -= 1;
            }
            if !skipped {
                prev.push((dir_ndx, n, bottom + lines.len()));
                for delta in 30..2000 {
                    if prev
                        .iter()
                        .rev()
                        .map(|(ndx, _, _)| ndx)
                        .skip(delta)
                        .take(delta)
                        .eq(prev.iter().rev().map(|(ndx, _, _)| ndx).take(delta))
                    {
                        let (_, prev_n, prev_bottom) = prev[prev.len() - delta - 1];
                        let bottom_delta = (bottom + lines.len()) - prev_bottom;
                        let factor = (STEPS_PT2 - prev_n) / delta;
                        n = prev_n + factor * delta;
                        bottom = prev_bottom + factor * bottom_delta - lines.len();
                        skipped = true;
                        continue;
                    }
                }
            }
            // if n % 10_000_000 == 0 {
            //     eprintln!("{}%", n * 100 / STEPS_PT2);
            //     let trimmed = trim_lines(&mut lines);
            //     dbg!(trimmed);
            //     dbg!(lines.len());
            //     bottom += trimmed;
            // }
            n += 1;
        }
        let n_empty = lines.iter().rev().take_while(|ln| ln.0 == 0).count();
        Ok(Num((lines.len() - n_empty + bottom) as u64))
    }
}
