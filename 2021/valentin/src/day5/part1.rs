use std::cmp::{max, min};
use std::collections::HashSet;
use std::{fs, ops};
use std::ops::Range;
use crate::helper::intersect_1d;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, _rhs: Point) -> Point {
        Point {
            x:self.x + _rhs.x,
            y:self.y + _rhs.y,
        }
    }
}

pub fn point_from(string: &str) -> Point {
    let split: Vec<&str> = string.split(',').collect();
    assert_eq!(split.len(), 2);
    return Point {
        x: split[0].parse::<i32>().unwrap(),
        y: split[1].parse::<i32>().unwrap(),
    };
}

pub struct Segment {
    p1: Point,
    p2: Point,
}

pub fn segment_from(line: &str) -> Segment {
    let split:Vec<&str> = line.split(" -> ").collect();
    return Segment {
        p1: point_from(split[0]),
        p2: point_from(split[1]),
    };
}


impl Segment {

     pub(crate) fn get_ranges(&self) -> (Range<i32>, Range<i32>) {
        let range_x = min(self.p1.x, self.p2.x)..(max(self.p1.x, self.p2.x) + 1);
        let range_y = min(self.p1.y, self.p2.y)..(max(self.p1.y, self.p2.y) + 1);
        return (range_x, range_y);
    }
    fn is_vertical(&self) -> bool {
        return self.p1.x == self.p2.x;
    }

    fn is_horizontal(&self) -> bool {
        return self.p1.y == self.p2.y;
    }

    fn get_dx(&self) -> i32 {
        if self.p2.x > self.p1.x {1} else if self.p2.x == self.p1.x { 0 } else {-1}
    }

    fn get_dy(&self) -> i32 {
        if self.p2.y > self.p1.y {1} else if self.p2.y == self.p1.y { 0 } else {-1}
    }

    pub(crate) fn intersect(&self, other: &Segment) -> bool {
        let (range_x, range_y) = self.get_ranges();
        let (other_range_x, other_range_y) = other.get_ranges();
        return intersect_1d(range_x, other_range_x) && intersect_1d(range_y, other_range_y);
    }

    fn simple_intersection_points(&self, other: &Segment) -> Vec<Point> {
        let mut res:Vec<Point> = Vec::new();
        let (range_x, range_y) = self.get_ranges();
        let (other_range_x, other_range_y) = other.get_ranges();
        for x in range_x {
            for y in range_y.clone() {
                if other_range_x.contains(&x) && other_range_y.contains(&y) {
                    res.push(Point { x, y, });
                }
            }
        }
        return res;
    }

    pub(crate) fn get_all_points(&self) -> HashSet<Point> {
        let mut res:HashSet<Point> = HashSet::new();
        let mut point = self.p1;
        res.insert(point);
        let mv = Point {
            x:self.get_dx(),
            y:self.get_dy(),
        };
        while point != self.p2 {
            point = point + mv;
            res.insert(point);
        }
        return res;
    }

}

pub fn solution() -> usize {
    let text = fs::read_to_string("src/day5/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim().split('\n').collect();
    let segments: Vec<Segment> = lines.into_iter()
        .map(|line| segment_from(line))
        .filter(|seg| seg.is_horizontal() || seg.is_vertical())
        .collect();
    let mut intersection_points:HashSet<Point> = HashSet::new();
    for i in 0..(segments.len() - 1) {
        for j in (i + 1)..segments.len() {
            if segments[i].intersect(&segments[j]) {
                let points = segments[i].simple_intersection_points(&segments[j]);
                for point in points {
                    intersection_points.insert(point);
                }
            }
        }
    }
    return intersection_points.len();
}