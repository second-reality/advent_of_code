use std::collections::HashSet;
use std::fs;
use crate::helper::parse_matrix_of_digits;


const N_COLUMNS:i32 = 100;
const N_LINES:i32 = 100;

fn get_neighbors(numbers:&Vec<u32>, index:i32) -> Vec<u32> {
    let (i, j) = (index / N_COLUMNS, index % N_COLUMNS);
    let mut res:Vec<u32> = Vec::new();
    for d in (-1..2).step_by(2) {
        // check neighbors up and down
        if (0..N_LINES).contains(&(i + d)) {
            res.push(numbers[(index + d* N_COLUMNS) as usize]);
        }
        // check neighbors left and right
        if (0..N_COLUMNS).contains(&(j + d)) {
            res.push(numbers[(index + d) as usize]);
        }
    }
    return res;
}
pub fn solution() -> u32 {
    let text = fs::read_to_string("src/day9/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim().split('\n').collect();

    let numbers:Vec<u32> = parse_matrix_of_digits(lines);
    let mut risk_level_sum:u32 = 0;
    for i in 0..numbers.len() {
        if get_neighbors(&numbers, i as i32).into_iter().all(|neighbor| neighbor > numbers[i]) {
            risk_level_sum += 1 + numbers[i];
        }
    };
    return risk_level_sum;
}