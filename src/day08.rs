use aoc_framework::*;

pub struct Day8;

impl_day!(Day8::{Part1, Part2}: 2022[8], r"
30373
25512
65332
33549
35390
");

const DELTAS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

struct Grid {
    data: Vec<u8>,
    w: usize,
    h: usize,
}

impl Grid {
    fn parse(input: impl Iterator<Item = String>) -> Grid {
        let mut input = input.peekable();
        let w = input.peek().map(|l| l.len()).unwrap_or(0);
        let data = input.fold(Vec::new(), |mut data, l| {
            data.extend(l.bytes().map(|b| b - b'0'));
            data
        });
        let h = data.len() / w;
        Grid { data, w, h }
    }

    fn get(&self, x: isize, y: isize) -> Option<u8> {
        if x < 0 || y < 0 || x as usize >= self.w || y as usize >= self.h {
            return None;
        }
        self.data.get(y as usize * self.w + x as usize).copied()
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        let cur_x = x as isize;
        let cur_y = y as isize;
        let cur = self.get(cur_x, cur_y).unwrap();
        'outer: for (dx, dy) in DELTAS {
            let mut x = cur_x + dx;
            let mut y = cur_y + dy;
            while let Some(val) = self.get(x, y) {
                if val >= cur {
                    continue 'outer;
                }
                x += dx;
                y += dy;
            }
            return true;
        }
        false
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let cur_x = x as isize;
        let cur_y = y as isize;
        let cur = self.get(cur_x, cur_y).unwrap();
        let mut total_score = 1;
        for (dx, dy) in DELTAS {
            let mut x = cur_x + dx;
            let mut y = cur_y + dy;
            let mut dir_score = 0;
            while let Some(val) = self.get(x, y) {
                dir_score += 1;
                if val >= cur {
                    break;
                }
                x += dx;
                y += dy;
            }
            total_score *= dir_score;
        }
        total_score
    }

    fn iter(&self) -> GridIter<'_> {
        GridIter {
            grid: self,
            x: 0,
            y: 0,
        }
    }
}

struct GridIter<'a> {
    grid: &'a Grid,
    x: usize,
    y: usize,
}

impl<'a> Iterator for GridIter<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = (self.x, self.y);
        if x >= self.grid.w || y >= self.grid.h {
            return None;
        }
        self.x += 1;
        if self.x == self.grid.w {
            self.x = 0;
            self.y += 1;
        }
        Some((x, y))
    }
}

pub struct Part1;

impl Part for Part1 {
    type D = Day8;
    const N: u8 = 1;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(21));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let grid = Grid::parse(input);
        Ok(Num(
            grid.iter().filter(|&(x, y)| grid.is_visible(x, y)).count() as u64,
        ))
    }
}

pub struct Part2;

impl Part for Part2 {
    type D = Day8;
    const N: u8 = 2;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(8));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let grid = Grid::parse(input);
        Ok(Num(grid
            .iter()
            .map(|(x, y)| grid.scenic_score(x, y))
            .max()
            .unwrap_or(0) as u64))
    }
}
