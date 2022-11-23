use std::fs;
use crate::day13::part1::{apply_instruction, parse_points_and_instructions, Point};

pub fn solution() -> usize {
    let text = fs::read_to_string("src/day13/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim().split('\n').collect();
    let (mut points, instructions) = parse_points_and_instructions(lines);
    for instruction in instructions {
        points = apply_instruction(instruction, points);
    }
    println!("{:?}", points);
    let x_max = points.iter().map(|p| (*p).x).max().unwrap();
    let y_max = points.iter().map(|p| (*p).y).max().unwrap();
    println!("x_max: {}, y_max: {}", x_max, y_max);
    for y in 0..(y_max+1) {
        for x in 0..(x_max+1) {
            let p = Point {x, y};
            if points.contains(&p) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    points.len()
}