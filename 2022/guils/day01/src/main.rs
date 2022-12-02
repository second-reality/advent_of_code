use itertools::sorted;
use std::fs;

//const INPUT: &str = "test.txt";
const INPUT: &str = "input.txt";

fn read(filename: &str) -> String {
    fs::read_to_string(filename).expect("failed to read")
}

fn split(content: String, delim: &str) -> Vec<String> {
    let split: Vec<String> = content
        .split(delim)
        .map(|s: &str| s.trim().to_string())
        .collect();
    split
}

fn read_input(filename: &str) -> Vec<String> {
    let content = read(filename);
    split(content, "\n\n")
}

fn map_input(lines: Vec<String>) -> Vec<i32> {
    let powers: Vec<i32> = lines
        .iter()
        .map(|x| {
            let items = split(x.to_string(), "\n");
            let sum: i32 = items.iter().map(|x| x.parse::<i32>().unwrap()).sum();
            sum
        })
        .collect();
    powers
}

fn maxarg(vals: Vec<i32>) -> (usize, i32) {
    let (idx, &max) = vals.iter().enumerate().max_by_key(|(_, &v)| v).unwrap();
    (idx, max)
}

fn step1() {
    let lines = read_input(INPUT);
    let powers = map_input(lines);
    let (_, max) = maxarg(powers);
    println!("step1: {max}");
}

fn step2() {
    let lines = read_input(INPUT);
    let powers = map_input(lines);
    let tot_of_3: i32 = sorted(powers).rev().take(3).sum();
    println!("step2: {tot_of_3}");
}

fn main() {
    step1();
    step2();
}
