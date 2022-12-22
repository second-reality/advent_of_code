use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    ops::{Add, Mul},
};

#[derive(Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

const UP: Coord = Coord { x: 0, y: 0, z: 1 };
const DOWN: Coord = Coord { x: 0, y: 0, z: -1 };
const RIGHT: Coord = Coord { x: 1, y: 0, z: 0 };
const LEFT: Coord = Coord { x: -1, y: 0, z: 0 };
const FRONT: Coord = Coord { x: 0, y: 1, z: 0 };
const BACK: Coord = Coord { x: 0, y: -1, z: 0 };

impl Add for Coord {
    type Output = Coord;
    fn add(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul<i32> for Coord {
    type Output = Coord;
    fn mul(self, rhs: i32) -> Self::Output {
        Coord {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Coord {
    fn manhattan(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
    fn adjacents(self) -> [Coord; 6] {
        [
            self + UP,
            self + DOWN,
            self + LEFT,
            self + RIGHT,
            self + FRONT,
            self + BACK,
        ]
    }
    fn is_trapped(self, droplets: &HashSet<Coord>) -> bool {
        let corner = Coord { x: 0, y: 0, z: 0 };
        // TODO DFS to reach max_iter point
        let mut prio_queue = BinaryHeap::from([Reverse((0, self))]);
        let mut visited = HashSet::from([self]);
        while let Some(Reverse((_, cur))) = prio_queue.pop() {
            if cur == corner {
                return false;
            }
            for adj in cur.adjacents() {
                if (!droplets.contains(&adj)) && (!visited.contains(&adj)) {
                    visited.insert(adj);
                    prio_queue.push(Reverse((adj.manhattan(&corner), adj)));
                }
            }
        }
        true
    }
}
fn parse(input: String) -> HashSet<Coord> {
    input
        .trim()
        .split('\n')
        .map(|line| {
            let mut tmp = line.split(',');
            let x: i32 = tmp.next().unwrap().parse().unwrap();
            let y: i32 = tmp.next().unwrap().parse().unwrap();
            let z: i32 = tmp.next().unwrap().parse().unwrap();
            Coord { x, y, z }
        })
        .collect()
}

pub fn part1(input: String) -> usize {
    let droplets = parse(input);
    droplets
        .iter()
        .map(|&coord| {
            coord
                .adjacents()
                .iter()
                .filter(|c| !droplets.contains(c))
                .count()
        })
        .sum()
}

pub fn part2(input: String) -> usize {
    let droplets = parse(input);
    droplets
        .iter()
        .map(|&coord| {
            coord
                .adjacents()
                .iter()
                .filter(|c| (!droplets.contains(c)) && (!(*c).is_trapped(&droplets)))
                .count()
        })
        .sum()
}

pub const EXPECTED1: usize = 64;
pub const EXPECTED2: usize = 58;
