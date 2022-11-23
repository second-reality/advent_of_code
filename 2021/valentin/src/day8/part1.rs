use std::collections::HashSet;
use std::fs;

pub fn solution() -> usize {
    let text = fs::read_to_string("src/day8/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim().split('\n').collect();
    let easy_digits_len:HashSet<usize> = HashSet::from([2, 4, 3, 7]);
    let mut count1478:usize = 0;
    for line in lines {
        let split:Vec<&str> = line.split(" | ").collect();
        let output:Vec<&str> = split[1].split(" ").collect();
        count1478 += output.into_iter()
            .map(|segments| segments.len())
            .filter(|size| easy_digits_len.contains(size))
            .count();
    }
    return count1478;
}