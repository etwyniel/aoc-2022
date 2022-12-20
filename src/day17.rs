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

#[derive(Debug, PartialEq, Eq)]
enum Dir {
    Left,
    Right,
}

fn clashes(lines: &[Line], pos: usize, shape: &[Line], offset: usize) -> bool {
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

fn print_lines(lines: &[Line]) {
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

fn print_shape(shape: &[Line], offset: usize) {
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
        let mut lines: Vec<Line> = Vec::new();
        let mut dir_ndx = 0;
        for n in 0..TOTAL_STEPS {
            let mut offset: usize = 2;
            let shape = SHAPES[n % SHAPES.len()];
            // print_shape(shape, offset);
            let mut n_empty = lines.iter().rev().take_while(|ln| ln.0 == 0).count();
            let top = n_empty;
            for _ in top..(shape.len() + 3) {
                lines.push(Line(0));
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
        }
        let n_empty = lines.iter().rev().take_while(|ln| ln.0 == 0).count();
        Ok(Num((lines.len() - n_empty) as u64))
    }
}

pub struct Part2;

impl Part for Part2 {
    type D = Day17;
    const N: u8 = 2;
    const EXAMPLE_RESULT: Option<Answer> = None;

    fn run(_input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        Ok(Num(0))
    }
}
