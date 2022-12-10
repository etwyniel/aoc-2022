use aoc_framework::*;

pub struct Day10;

impl_day!(Day10::{Part1, Part2}: 2022[10], r"
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
");

struct Proc {
    cycle: i64,
    x: i64,
    result: i64,
    buffer: Vec<bool>,
}

impl Proc {
    fn tick(&mut self) {
        if self.cycle == 20 || (self.cycle > 20 && (self.cycle - 20) % 40 == 0) {
            self.result += self.x * self.cycle;
        }
        self.cycle += 1;
    }

    fn update_buffer(&mut self) {
        let x = (self.cycle - 1) % 40;
        let y = (self.cycle - 1) / 40;
        if y > 5 {
            return;
        }
        if (x - self.x).abs() < 2 {
            self.buffer[self.cycle as usize - 1] = true;
        }
    }
}

pub struct Part1;

impl Part for Part1 {
    type D = Day10;
    const N: u8 = 1;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(13140));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let mut proc = Proc {
            cycle: 1,
            x: 1,
            result: 0,
            buffer: Vec::new(),
        };
        for line in input {
            if let Some(arg) = line.strip_prefix("addx ") {
                let val = arg.parse::<i64>()?;
                proc.tick();
                proc.tick();
                proc.x += val;
            } else {
                proc.tick();
            }
        }
        Ok(Num(proc.result as u64))
    }
}

pub struct Part2;

impl Part for Part2 {
    type D = Day10;
    const N: u8 = 2;
    const EXAMPLE_RESULT: Option<Answer> = None;

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let mut proc = Proc {
            cycle: 1,
            x: 1,
            result: 0,
            buffer: vec![false; 40 * 6],
        };
        for line in input {
            if let Some(arg) = line.strip_prefix("addx ") {
                let val = arg.parse::<i64>()?;
                proc.update_buffer();
                proc.tick();
                proc.update_buffer();
                proc.tick();
                proc.x += val;
            } else {
                proc.update_buffer();
                proc.tick();
            }
        }
        let mut out = String::with_capacity(50);
        for (i, pix) in proc.buffer.iter().enumerate() {
            if i % 40 == 0 {
                out.push('\n');
            }
            if *pix {
                out.push('#');
            } else {
                out.push('.');
            }
        }
        Ok(Str(std::borrow::Cow::Owned(out)))
    }
}
