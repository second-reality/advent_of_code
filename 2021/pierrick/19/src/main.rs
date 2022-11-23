use itertools::*;
use std::collections::HashSet;
use std::ops::Sub;

const INPUT: &str = include_str!("../input");
const TEST: &str = include_str!("../test");

fn get_input(s: &str) -> Vec<Report> {
    let mut res = vec![];
    for line in s.lines().filter(|l| !l.is_empty()) {
        if line.starts_with("---") {
            res.push(Report::new());
        } else {
            let coords = line.split(',');
            let coords: Vec<i32> = coords.map(|c| c.parse().unwrap()).collect();
            res.last_mut()
                .unwrap()
                .add_point(Point::from_slice(&coords));
        }
    }

    res
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone, Default)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        Point { x, y, z }
    }
}

impl Point {
    fn distance_to(&self, other: &Self) -> i32 {
        (other.z - self.z).abs() + (other.y - self.y).abs() + (other.x - self.x).abs()
    }
}

const NUM_PERMUTATIONS: usize = 48;

impl Point {
    fn from_slice(coords: &[i32]) -> Self {
        assert_eq!(3, coords.len());
        Point {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
    }

    fn all_rotations(&self) -> Vec<Point> {
        let mut res = vec![];
        let (x, y, z) = (self.x, self.y, self.z);

        let mut add_permutations = |coords: [i32; 3]| {
            coords
                .into_iter()
                .permutations(3)
                .for_each(|perm| res.push(Point::from_slice(&perm)));
        };

        add_permutations([x, y, z]);
        add_permutations([x, y, -z]);
        add_permutations([x, -y, z]);
        add_permutations([x, -y, -z]);
        add_permutations([-x, y, z]);
        add_permutations([-x, y, -z]);
        add_permutations([-x, -y, z]);
        add_permutations([-x, -y, -z]);

        // every permutation has 6 possibilities -> 48 results
        assert_eq!(NUM_PERMUTATIONS, res.len());

        res
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Report {
    reference: Point,
    points: HashSet<Point>,
}

impl Report {
    fn new() -> Self {
        Report {
            reference: Point::default(),
            points: HashSet::new(),
        }
    }

    fn from_reference(&self, reference: Point) -> Report {
        Report {
            reference,
            points: self.points.iter().map(|p| *p - reference).collect(),
        }
    }

    fn all_rotations(&self) -> Vec<Report> {
        let mut res = vec![Report::new(); NUM_PERMUTATIONS];
        for p in self.points.iter() {
            let rot = p.all_rotations();
            for (index, r) in rot.into_iter().enumerate() {
                res[index].add_point(r);
            }
        }
        res
    }

    fn points_common(&self, rhs: &Report) -> usize {
        self.points.intersection(&rhs.points).count()
    }

    fn add_point(&mut self, p: Point) {
        self.points.insert(p);
    }

    fn matching_reference(&self, reference: &Report) -> Option<(Report, usize)> {
        let rotated = self.all_rotations();
        for r in rotated {
            for p in r.points.iter() {
                for pr in reference.points.iter() {
                    let position = *p - *pr;
                    let candidate = r.from_reference(position);
                    let num_common = reference.points_common(&candidate);
                    if num_common >= 12 {
                        return Some((candidate, num_common));
                    }
                }
            }
        }
        None
    }
}

fn all_points(all: &[Report]) -> (HashSet<Point>, i32) {
    let mut reference = all[0].clone();

    let mut positions = vec![];

    let mut todo: Vec<Report> = all.to_vec();

    while !todo.is_empty() {
        let mut next = None;
        for (index, report) in todo.iter().enumerate() {
            if let Some((report, count)) = report.matching_reference(&reference) {
                next = Some((index, (report, count)));
                break;
            }
        }
        let (index, (report, count)) = next.unwrap();
        todo.remove(index);
        reference.points.extend(report.points.iter());
        positions.push(report.reference);
        println!(
            "{}/{} scanners - {} points found (+{}) - {:?}",
            all.len() - todo.len(),
            all.len(),
            reference.points.len(),
            count,
            report.reference
        );
    }

    // part2
    let max_dist = positions
        .iter()
        .permutations(2)
        .map(|points| {
            let p1 = points[0];
            let p2 = points[1];
            p1.distance_to(p2)
        })
        .max()
        .unwrap();

    (reference.points, max_dist)
}

fn main() {
    let input = get_input(INPUT);
    let test = get_input(TEST);

    assert_eq!(79, all_points(&test).0.len());
    let res = all_points(&input);
    println!("{}", res.0.len());
    println!("{}", res.1);
}
