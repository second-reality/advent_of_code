use std::collections::HashSet;
use std::fs;
use crate::day4::part1::{bingo, parse_matrices};

const MAT_N: usize = 5;
const N: usize = 100;


pub fn solution() -> i32 {
    let text = fs::read_to_string("src/day4/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<String> = text.trim().split('\n').map(|line| line.trim().replace("  ", " ")).collect();
    println!("{}", lines[38]);

    let numbers_to_cross: Vec<i32> = lines[0].split(',').map(|s| s.parse::<i32>().unwrap()).collect();
    let grids = parse_matrices(&lines);
    let mut set: HashSet<i32> = HashSet::new();
    for i in 0..N {
        set.insert(numbers_to_cross[i]);
        let losers: Vec<&Vec<Vec<i32>>> = grids.iter()
            .filter(|grid| bingo(&grid, &set) == -1)
            .collect();
        if losers.len() == 1 {
            let grid = losers[0];

            let mut j = i;
            let mut sum_unmarked = bingo(grid, &set);
            while sum_unmarked == -1 {
                j += 1;
                set.insert(numbers_to_cross[j]);
                sum_unmarked = bingo(grid, &set);
            }
            println!("Last grid {:?} finally wins with {} th number: {}, unmarked sum {}", grid, j, numbers_to_cross[j], sum_unmarked);
            return sum_unmarked * numbers_to_cross[j];
        }
    }
    println!("Probably error, no one wins");
    return -1;
}