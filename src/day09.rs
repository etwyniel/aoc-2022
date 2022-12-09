use std::{
    collections::HashSet,
    ops::{AddAssign, Sub},
};

use aoc_framework::{anyhow::bail, *};

pub struct Day9;

impl_day!(Day9::{Part1, Part2}: 2022[9], r"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
");

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Point {
    fn abs(self) -> Point {
        Point {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    fn signum(self) -> Point {
        Point {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }
}

struct Rope(Vec<Point>);

impl Rope {
    fn step(&mut self, delta: Point) -> Point {
        self.0[0] += delta;
        let mut prev = self.0[0];
        for knot in self.0.iter_mut().skip(1) {
            let delta = prev - *knot;
            match delta.abs() {
                Point { x: 0 | 1, y: 0 | 1 } => break,
                _ => {
                    *knot += delta.signum();
                    prev = *knot;
                }
            }
        }
        self.0.last().copied().unwrap()
    }
}

fn parse_line(line: &str) -> anyhow::Result<(Point, usize)> {
    let (dir, dist) = line.split_once(' ').unwrap();
    let (x, y) = match dir.as_bytes()[0] {
        b'L' => (-1, 0),
        b'U' => (0, 1),
        b'R' => (1, 0),
        b'D' => (0, -1),
        _ => bail!("Invalid direction {dir}"),
    };
    Ok((Point { x, y }, dist.parse()?))
}

fn count_tail_positions(
    rope_len: usize,
    input: impl Iterator<Item = String>,
) -> anyhow::Result<Answer> {
    let mut rope = Rope(vec![Point::default(); rope_len]);
    let mut tail_positions = HashSet::new();
    tail_positions.insert(Point::default());
    for line in input {
        let (delta, dist) = parse_line(&line)?;
        for _ in 0..dist {
            let tail = rope.step(delta);
            tail_positions.insert(tail);
        }
    }
    Ok(Num(tail_positions.len() as u64))
}

pub struct Part1;

impl Part for Part1 {
    type D = Day9;
    const N: u8 = 1;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(88));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        count_tail_positions(2, input)
    }
}

pub struct Part2;

impl Part for Part2 {
    type D = Day9;
    const N: u8 = 2;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(36));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        count_tail_positions(10, input)
    }
}
