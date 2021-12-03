use std::collections::HashMap;

fn part1(input: &str) -> i32 {
    let mut scores: Vec<HashMap<char, i32>> = vec![];

    for l in input.lines() {
        assert!(scores.is_empty() || l.len() == scores.len());
        scores.resize(l.len(), HashMap::new());

        for (idx, c) in l.chars().enumerate() {
            *scores[idx].entry(c).or_insert(0) += 1;
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for s in scores {
        let most_frequent_bit = if s[&'0'] > s[&'1'] { 0 } else { 1 };
        let least_frequent_bit = !most_frequent_bit & 0x1;
        gamma <<= 1;
        epsilon <<= 1;
        gamma += most_frequent_bit;
        epsilon += least_frequent_bit;
    }

    gamma * epsilon
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part1(input));
}
