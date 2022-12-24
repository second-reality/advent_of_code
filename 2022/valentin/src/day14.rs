use std::{collections::HashSet, str::FromStr};

type Coord = (i32, i32);
trait FallCoord {
    fn diag_left(&self) -> Self;
    fn diag_right(&self) -> Self;
    fn down(&self) -> Self;
    fn from_input_str(s: &str) -> Self;
    fn range_inclusive(self, other: &Self) -> Vec<Coord>;
}

impl FallCoord for Coord {
    fn diag_left(&self) -> Self {
        (self.0 - 1, self.1 + 1)
    }

    fn diag_right(&self) -> Self {
        (self.0 + 1, self.1 + 1)
    }

    fn down(&self) -> Self {
        (self.0, self.1 + 1)
    }
    fn from_input_str(s: &str) -> Self {
        let mut tmp = s.split(',');
        (
            tmp.next().unwrap().parse().unwrap(),
            tmp.next().unwrap().parse().unwrap(),
        )
    }

    fn range_inclusive(self, other: &Self) -> Vec<Coord> {
        if self.0 == other.0 {
            let (mn, mx) = (self.1.min(other.1), self.1.max(other.1));
            (mn..=mx).map(|y| (self.0, y)).collect()
        } else {
            let (mn, mx) = (self.0.min(other.0), self.0.max(other.0));
            (mn..=mx).map(|x| (x, self.1)).collect()
        }
    }
}

const SAND_SOURCE: Coord = (500, 0);

#[derive(Debug)]
struct Cave {
    walls: HashSet<Coord>,
    lowest_wall: i32,
}

enum FallingState {
    Finished(Coord),
    Endless,
    Error,
}

impl Cave {
    fn fill(&self, sand_pos: Coord) -> FallingState {
        if self.walls.contains(&sand_pos) {
            FallingState::Error
        } else if sand_pos.1 > self.lowest_wall {
            FallingState::Endless
        } else {
            let to_try = [sand_pos.down(), sand_pos.diag_left(), sand_pos.diag_right()];
            to_try
                .into_iter()
                .find_map(|pos| match self.fill(pos) {
                    FallingState::Error => None,
                    other => Some(other),
                })
                .unwrap_or(FallingState::Finished(sand_pos))
        }
    }
    fn fill_until_end(&mut self) -> usize {
        let mut res = 0;
        while let FallingState::Finished(new_wall) = self.fill(SAND_SOURCE) {
            self.walls.insert(new_wall);
            res += 1;
        }
        res
    }
}
impl FromStr for Cave {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut walls: HashSet<Coord> = HashSet::new();
        for line in s.split('\n') {
            let mut points = line.split(" -> ").map(Coord::from_input_str);
            let mut prev = points.next().unwrap();
            for p in points {
                let range = prev.range_inclusive(&p);
                walls.extend(range);
                prev = p;
            }
        }
        let lowest_wall = *walls.iter().map(|(_, y)| y).max().unwrap();
        Ok(Cave { walls, lowest_wall })
    }
}
pub fn part1(input: String) -> usize {
    let mut cave: Cave = input.trim().parse().unwrap();
    cave.fill_until_end()
}

pub fn part2(input: String) -> usize {
    let mut cave: Cave = input.trim().parse().unwrap();
    cave.lowest_wall += 2;
    let min_x = SAND_SOURCE.0 - cave.lowest_wall - 1;
    let max_x = SAND_SOURCE.0 + cave.lowest_wall + 1;
    cave.walls
        .extend((min_x..=max_x).map(|x| (x, cave.lowest_wall)));
    cave.fill_until_end()
}

pub const EXPECTED1: usize = 24;
pub const EXPECTED2: usize = 93;
