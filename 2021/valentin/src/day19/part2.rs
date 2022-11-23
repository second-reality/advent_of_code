use std::fs;
use crate::helper::sub_sum;

pub fn solution() -> usize {
    let text = fs::read_to_string("src/day1/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim()
        .split('\n')
        .collect();
    let numbers: Vec<i32> = lines.into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
    let count_increase = (1..(numbers.len()-2)).into_iter().reduce(|acc, i| {
        if sub_sum(i-1, i + 2, &numbers) < sub_sum(i, i + 3, &numbers) { acc + 1 } else { acc }
    });
    return count_increase.expect("error");
}