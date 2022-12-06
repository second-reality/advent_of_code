use std::collections::HashSet;

//const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");

fn read_input() -> String {
    INPUT.trim().to_string()
}

fn find_marker(input: &String, size: usize) -> usize {
    for i in 0..input.len() {
        let chars = input[i..i + size].chars().collect::<HashSet<_>>();
        if chars.len() == size {
            return i + size;
        }
    }
    panic!();
}

fn step1() {
    let input = read_input();
    let res = find_marker(&input, 4);
    println!("step1: {res}");
}

fn step2() {
    let input = read_input();
    let res = find_marker(&input, 14);
    println!("step2: {res}");
}

fn main() {
    step1();
    step2();
}
