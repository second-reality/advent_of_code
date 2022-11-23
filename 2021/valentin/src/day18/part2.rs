use std::fs;
use crate::day18::part1::FishNumber;

pub fn solution() -> usize {
    let text = fs::read_to_string("src/day18/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim().split('\n').collect();
    let mut max_magnitude = 0;
    for i in 0..lines.len() {
        for j in 0..lines.len() {
            let mut sum = FishNumber::from(lines[i], false, true) + FishNumber::from(lines[j], false, true);
            sum.reduce();
            let magnitude = sum.magnitude();
            if magnitude > max_magnitude {
                max_magnitude = magnitude;
            }
        }
    }
    max_magnitude
}