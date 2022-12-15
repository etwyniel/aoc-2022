use std::str::FromStr;

use aoc_framework::{anyhow::anyhow, *};

pub struct Day15;

impl_day!(Day15::{Part1, Part2}: 2022[15], r"
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn dist(&self, rhs: &Point) -> u64 {
        self.x.abs_diff(rhs.x) + self.y.abs_diff(rhs.y)
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(", ")
            .and_then(|(x, y)| {
                x.strip_prefix("x=")
                    .and_then(|x| y.strip_prefix("y=").map(|y| (x, y)))
            })
            .ok_or_else(|| anyhow!("Invalid input"))?;
        Ok(Point {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

#[derive(Debug)]
struct Sensor {
    pos: Point,
    beacon: Point,
    beacon_dist: u64,
}

impl FromStr for Sensor {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s
            .split_once(':')
            .and_then(|(l, r)| {
                l.rsplit("at ")
                    .next()
                    .and_then(|l| r.rsplit("at ").next().map(|r| (l, r)))
            })
            .ok_or_else(|| anyhow!("Invalid input"))?;
        let pos: Point = l.parse()?;
        let beacon: Point = r.parse()?;
        let beacon_dist = pos.dist(&beacon);
        Ok(Sensor {
            pos,
            beacon,
            beacon_dist,
        })
    }
}

pub struct Part1;

impl Part for Part1 {
    type D = Day15;
    const N: u8 = 1;
    const EXAMPLE_RESULT: Option<Answer> = None; //Some(Num(26));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let sensors: Vec<_> = input
            .map(|line| line.parse::<Sensor>())
            .collect::<anyhow::Result<_>>()?;
        let mut beacons = Vec::new();
        sensors.iter().for_each(|sensor| {
            if !beacons.contains(&sensor.beacon) {
                beacons.push(sensor.beacon);
            }
        });
        let max_x = sensors
            .iter()
            .map(|sensor| sensor.pos.x + sensor.beacon_dist as i64)
            .max()
            .unwrap();
        let min_x = sensors
            .iter()
            .map(|sensor| sensor.pos.x - sensor.beacon_dist as i64)
            .min()
            .unwrap();
        dbg!((min_x..=max_x).count());
        let mut impossible = 0;
        let y = 2_000_000;
        // let y = 10;
        for x in min_x..=max_x {
            let p = Point { x, y };
            if beacons.contains(&p) {
                continue;
            }
            if sensors
                .iter()
                .any(|sensor| sensor.pos.dist(&p) <= sensor.beacon_dist)
            {
                impossible += 1;
            }
        }
        Ok(Num(impossible))
    }
}

pub struct Part2;

impl Part for Part2 {
    type D = Day15;
    const N: u8 = 2;
    const EXAMPLE_RESULT: Option<Answer> = None;

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let sensors: Vec<_> = input
            .map(|line| line.parse::<Sensor>())
            .collect::<anyhow::Result<_>>()?;
        const LIMIT: i64 = 4_000_000;
        for y in 0..=LIMIT {
            let mut x = 0;
            'xloop: while x <= LIMIT {
                for sensor in &sensors {
                    let sensor_dist = sensor.pos.dist(&Point { x, y });
                    if sensor_dist <= sensor.beacon_dist {
                        if x < sensor.pos.x {
                            x += (sensor.pos.x - x) / 2 + 1;
                        } else {
                            let ydiff = y.abs_diff(sensor.pos.y) as i64;
                            x = sensor.pos.x + sensor.beacon_dist as i64 - ydiff + 1;
                        }
                        continue 'xloop;
                    }
                }
                return Ok(Num((x * LIMIT + y) as u64));
            }
        }
        Ok(Num(0))
    }
}
