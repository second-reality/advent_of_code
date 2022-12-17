use std::{
    num::ParseIntError,
    ops::{Add, Mul, RangeInclusive},
    str::FromStr,
};
#[derive(PartialEq, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl FromStr for Coord {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tmp = s
            .split(' ')
            .filter(|sp| sp.contains('='))
            .map(|sp| sp.split('=').last().unwrap());
        let x: i32 = tmp.next().unwrap().replace(",", "").parse()?;
        let y: i32 = tmp.next().unwrap().parse()?;
        Ok(Coord { x, y })
    }
}

impl Coord {
    fn manhattan_dist(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<i32> for Coord {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        Coord {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

struct Circles {
    center: Coord,
    radius: i32,
}
impl Circles {
    fn intersection_with_y(&self, y: i32) -> Option<RangeInclusive<i32>> {
        let delta = self.radius - (y - self.center.y).abs();
        if delta >= 0 {
            Some((self.center.x - delta)..=(self.center.x + delta))
        } else {
            None
        }
    }

    fn contains(&self, point: &Coord) -> bool {
        self.center.manhattan_dist(point) <= self.radius
    }
}
fn union(intervals: &mut Vec<RangeInclusive<i32>>, range2: RangeInclusive<i32>) {
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    let to_remove: Vec<usize> = intervals
        .iter()
        .enumerate()
        .filter_map(|(i, range1)| {
            if range1.start() <= range2.end() && range1.end() >= range2.start() {
                min = *range1.start().min(range2.start()).min(&min);
                max = *range1.end().max(range2.end()).max(&max);
                Some(i)
            } else {
                None
            }
        })
        .collect();
    let add = if to_remove.is_empty() {
        range2
    } else {
        to_remove.into_iter().rev().for_each(|i| {
            intervals.remove(i);
        });
        min..=max
    };
    intervals.push(add);
}

fn parse_input(input: String, y_to_check: i32) -> (Vec<Circles>, Vec<Coord>) {
    let mut beacons_to_check = Vec::new();
    let res = input
        .trim()
        .split('\n')
        .map(|line| {
            let mut halfs = line.split(':');
            let scanner: Coord = halfs.next().unwrap().parse().unwrap();
            let beacon: Coord = halfs.next().unwrap().parse().unwrap();
            let dist = scanner.manhattan_dist(&beacon);
            if beacon.y == y_to_check && !beacons_to_check.contains(&beacon) {
                beacons_to_check.push(beacon)
            }
            Circles {
                center: scanner,
                radius: dist,
            }
        })
        .collect();
    (res, beacons_to_check)
}

pub fn part1(input: String) -> usize {
    let y_to_check = if input.len() < 800 {
        // example
        10
    } else {
        2_000_000
    };
    let (circles, beacons) = parse_input(input, y_to_check);
    let mut ranges_x: Vec<RangeInclusive<i32>> = Vec::new();
    circles
        .iter()
        .filter_map(|circle| circle.intersection_with_y(y_to_check))
        .for_each(|range| union(&mut ranges_x, range));
    let beacon_ytc: usize = beacons
        .into_iter()
        .filter(|b| ranges_x.iter().any(|range| range.contains(&b.x)))
        .count();
    let count: usize = ranges_x.into_iter().map(|interval| interval.count()).sum();
    count - beacon_ytc
}

const LEFT: Coord = Coord { x: -1, y: 0 };
const RIGHT: Coord = Coord { x: 1, y: 0 };
const UP: Coord = Coord { x: 0, y: -1 };
const DOWN: Coord = Coord { x: 0, y: 1 };

const DIAG_UP_LEFT: Coord = Coord { x: -1, y: -1 };
const DIAG_UP_RIGHT: Coord = Coord { x: 1, y: -1 };
const DIAG_DOWN_RIGHT: Coord = Coord { x: 1, y: 1 };
const DIAG_DOWN_LEFT: Coord = Coord { x: -1, y: 1 };

pub fn part2(input: String) -> usize {
    let len_search = if input.len() < 800 {
        // example
        20
    } else {
        4_000_000
    };
    let (circles, _) = parse_input(input, 0);

    let point = circles
        .iter()
        .find_map(|circle| {
            let r = circle.radius + 1;
            let mut contour = (0..=circle.radius).flat_map(|i| {
                [
                    circle.center + (UP * r) + (DIAG_DOWN_RIGHT * i),
                    circle.center + (RIGHT * r) + (DIAG_DOWN_LEFT * i),
                    circle.center + (DOWN * r) + (DIAG_UP_LEFT * i),
                    circle.center + (LEFT * r) + (DIAG_UP_RIGHT * i),
                ]
            });
            contour.find(|point| {
                (0..=len_search).contains(&point.x)
                    && (0..=len_search).contains(&point.y)
                    && circles.iter().all(|c| !c.contains(point))
            })
        })
        .unwrap();
    let (x, y) = (point.x as usize, point.y as usize);
    x * 4_000_000 + y
}

pub const EXPECTED1: usize = 26;
pub const EXPECTED2: usize = 56000011;
