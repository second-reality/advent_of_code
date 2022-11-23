use std::collections::{HashMap};
use std::fs;
use crate::day5::part1::{Point, Segment, segment_from};


pub fn solution() -> usize {
    let text = fs::read_to_string("src/day5/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim().split('\n').collect();
    let segments: Vec<Segment> = lines.into_iter()
        .map(|line| segment_from(line))
        .collect();
    let mut marked_points:HashMap<Point, u8> = HashMap::new();
    for segment in segments {
        let points = segment.get_all_points();
        for point in points {
            if marked_points.contains_key(&point) {
                marked_points.insert(point, marked_points.get(&point).unwrap() + 1);
            } else {
                marked_points.insert(point, 1);
            }
        }
    }
    return marked_points.iter().filter(|(_, v)| **v > 1).count();
}