use std::collections::BinaryHeap;

use aoc_framework::*;

pub struct Day12;

impl_day!(Day12::{Part1, Part2}: 2022[12], r"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
");

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Start,
    End,
    Height(u8),
}

impl Tile {
    fn elevation(self) -> u8 {
        match self {
            Tile::Start => 0,
            Tile::End => 25,
            Tile::Height(h) => h,
        }
    }

    fn dist(self, rhs: Tile) -> u64 {
        let i = self.elevation();
        let j = rhs.elevation();
        if i + 1 < j {
            10_000
        } else {
            1
        }
    }
}

struct Entry {
    pos: usize,
    priority: u64,
}

impl std::cmp::PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl std::cmp::Eq for Entry {}

impl std::cmp::PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.priority.partial_cmp(&self.priority)
    }
}

impl std::cmp::Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.priority.cmp(&self.priority)
    }
}

struct Grid {
    data: Vec<Tile>,
    w: usize,
    h: usize,
}

impl Grid {
    fn parse(input: impl Iterator<Item = String>) -> Grid {
        let mut input = input.peekable();
        let w = input.peek().map(|line| line.len()).unwrap_or(0);
        let data = input.fold(Vec::new(), |mut out, line| {
            out.extend(line.bytes().map(|b| match b {
                b'S' => Tile::Start,
                b'E' => Tile::End,
                b'a'..=b'z' => Tile::Height(b - b'a'),
                _ => panic!("Invalid input"),
            }));
            out
        });
        let h = data.len() / w;
        Grid { data, w, h }
    }
}

pub struct Part1;

impl Part for Part1 {
    type D = Day12;
    const N: u8 = 1;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(31));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let grid = Grid::parse(input);
        let len = grid.data.len();
        let source = grid
            .data
            .iter()
            .position(|&tile| tile == Tile::Start)
            .unwrap();
        let mut dist = vec![u64::MAX; len];
        dist[source] = 0;
        let mut prev = vec![None; len];
        let mut unvisited = len;
        let mut q = vec![true; len];
        let mut heap = BinaryHeap::with_capacity(len);
        for i in 0..len {
            let priority = if i == source { 0 } else { u64::MAX };
            heap.push(Entry { pos: i, priority });
        }
        while unvisited > 0 {
            let Entry { pos, priority } = heap.pop().unwrap();
            let pos = dist
                .iter()
                .enumerate()
                .filter(|(i, _)| q[*i])
                .min_by_key(|(_, v)| *v)
                .unwrap()
                .0;
            q[pos] = false;
            unvisited -= 1;
            if grid.data[pos] == Tile::End {
                let mut p = pos;
                let mut path_len = 0;
                // let mut path = Vec::new();
                while let Some(previous) = prev[p] {
                    // path.push(previous);
                    path_len += 1;
                    p = previous;
                }
                // for y in 0..grid.h {
                //     for x in 0..grid.w {
                //         let pos = x + y * grid.w;
                //         if path.contains(&pos) {
                //             print!("#");
                //         } else {
                //             print!("{}", match grid.data[pos] {
                //                 Tile::Start => 'S',
                //                 Tile::End => 'E',
                //                 Tile::Height(h) => (b'a' + h) as char,
                //             })
                //         }
                //     }
                //     println!();
                // }
                return Ok(Num(path_len as u64));
            }
            let x = pos % grid.w;
            let y = pos / grid.w;
            for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let nx = x as isize - dx;
                let ny = y as isize - dy;
                if nx < 0 || ny < 0 || nx >= grid.w as isize || ny >= grid.h as isize {
                    continue;
                }
                let npos = nx as usize + ny as usize * grid.w;
                if !q[npos] {
                    continue;
                }
                let alt = dist[pos] + grid.data[pos].dist(grid.data[npos]);
                if alt < dist[npos] {
                    dist[npos] = alt;
                    prev[npos] = Some(pos);
                }
            }
        }
        Ok(Num(0))
    }
}

pub struct Part2;

impl Part for Part2 {
    type D = Day12;
    const N: u8 = 2;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(29));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let grid = Grid::parse(input);
        let len = grid.data.len();
        let source = grid
            .data
            .iter()
            .position(|&tile| tile == Tile::End)
            .unwrap();
        let mut dist = vec![u64::MAX; len];
        dist[source] = 0;
        let mut prev = vec![None; len];
        let mut unvisited = len;
        let mut q = vec![true; len];
        while unvisited > 0 {
            let pos = dist
                .iter()
                .enumerate()
                .filter(|(i, _)| q[*i])
                .min_by_key(|(_, v)| *v)
                .unwrap()
                .0;
            q[pos] = false;
            unvisited -= 1;
            let x = pos % grid.w;
            let y = pos / grid.w;
            for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let nx = x as isize - dx;
                let ny = y as isize - dy;
                if nx < 0 || ny < 0 || nx >= grid.w as isize || ny >= grid.h as isize {
                    continue;
                }
                let npos = nx as usize + ny as usize * grid.w;
                if !q[npos] {
                    continue;
                }
                let alt = dist[pos] + grid.data[npos].dist(grid.data[pos]);
                if alt < dist[npos] {
                    dist[npos] = alt;
                    prev[npos] = Some(pos);
                }
            }
        }
        Ok(Num(grid
            .data
            .into_iter()
            .zip(dist.into_iter())
            .filter(|(tile, _)| tile.elevation() == 0)
            .map(|(_, dist)| dist)
            .min()
            .unwrap()))
    }
}
