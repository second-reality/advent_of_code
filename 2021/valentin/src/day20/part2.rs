use std::fs;
use crate::day20::part1::{count_pixels_on, parse_input, process_image};

pub fn solution() -> usize {
    let text = fs::read_to_string("src/day20/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim().split('\n').collect();
    let n_steps = 50;
    let (magic_line, mut image) = parse_input(lines, n_steps, 100);
    for i in 0..n_steps {
        image = process_image(&image, &magic_line);
    }
    count_pixels_on(&image)
}