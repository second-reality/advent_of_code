use std::fs;
use crate::day11::part1::step_and_count_flash;

const N:usize = 100;
pub fn solution() -> usize {
    let text = fs::read_to_string("src/day11/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim().split('\n').collect();

    let mut grid: Vec<u32> = lines.into_iter()
        .flat_map(|s|
            s.chars().map(|c| c.to_digit(10).unwrap()))
        .collect();
    let mut cur_step = 1;
    while step_and_count_flash(&mut grid) != N {
        cur_step += 1;
    }
    cur_step
}