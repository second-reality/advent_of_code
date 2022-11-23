use std::collections::HashSet;

use itertools::Itertools;

fn get_line(s: &str) -> (Vec<String>, Vec<String>) {
    let mut line = s.split(" | ");
    let left = line.next().unwrap();
    let left = left.split_whitespace().map(String::from).collect();
    let right = line.next().unwrap();
    let right = right.split_whitespace().map(String::from).collect();
    (left, right)
}

fn get_input(s: &str) -> Vec<(Vec<String>, Vec<String>)> {
    s.lines().map(get_line).collect()
}

fn get_possible_digit(s: &str, permutation: &[char]) -> Option<u8> {
    assert_eq!(7, permutation.len());
    let mut changed = String::from(s);
    for (index, &val) in permutation.iter().enumerate() {
        changed = changed.replace(&val.to_string(), &index.to_string());
    }
    changed = changed.chars().sorted().collect();

    //    0
    //  1   2
    //    3
    //  4   5
    //    6
    match changed.as_str() {
        "012456" => Some(0),
        "25" => Some(1),
        "02346" => Some(2),
        "02356" => Some(3),
        "1235" => Some(4),
        "01356" => Some(5),
        "013456" => Some(6),
        "025" => Some(7),
        "0123456" => Some(8),
        "012356" => Some(9),
        _ => None,
    }
}

fn possible_segments(s: &str) -> HashSet<Vec<char>> {
    let segments = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    let mut working = HashSet::new();
    segments
        .iter()
        .copied()
        .permutations(segments.len())
        .filter(|p| get_possible_digit(s, p).is_some())
        .for_each(|perm| {
            working.insert(perm.into_iter().collect());
        });
    working
}

fn is_unique_digit(s: &str) -> bool {
    // 0 -> 6 segments
    // 1 -> 2 segments (unique)
    // 2 -> 5 segments
    // 3 -> 5 segments
    // 4 -> 4 segments (unique)
    // 5 -> 5 segments
    // 6 -> 6 segments
    // 7 -> 3 segments (unique)
    // 8 -> 7 segments (unique)
    // 9 -> 6 segments

    let len = s.len();
    len == 2 || len == 4 || len == 3 || len == 7
}

fn unique_digits(words: &[Vec<String>]) -> usize {
    words
        .iter()
        .flatten()
        .filter(|s| is_unique_digit(s))
        .count()
}

fn unique_permutation(patterns: &[&str]) -> Vec<char> {
    let mut possible = possible_segments(patterns[0]);
    for s in patterns {
        possible = possible
            .intersection(&possible_segments(s))
            .cloned()
            .collect();
        //println!("From: {}: {:?}", s, blop);
    }
    assert_eq!(1, possible.len());
    let unique = possible.iter().next().unwrap();
    unique.clone()
}

fn right_part(v: &[(Vec<String>, Vec<String>)]) -> Vec<Vec<String>> {
    v.iter().map(|(_, r)| r).cloned().collect()
}

fn one_display((patterns, numbers): &(Vec<String>, Vec<String>)) -> i64 {
    let patterns: Vec<&str> = patterns.iter().map(String::as_str).collect();
    let perm = unique_permutation(&patterns);
    let digits: Vec<u8> = numbers
        .iter()
        .map(|s| get_possible_digit(s, &perm).unwrap())
        .collect();
    let mut res: i64 = 0;
    for d in digits {
        res *= 10;
        res += d as i64;
    }
    res
}

fn main() {
    let input = get_input(include_str!("../input.txt"));
    let test = get_input(include_str!("../test_input.txt"));
    println!("--------------------");
    println!("This code is shitty bruteforce approach, you should use:");
    println!("cargo run --release");
    println!("--------------------");
    println!("test {}", unique_digits(&right_part(&test)));
    println!("{}", unique_digits(&right_part(&input)));
    println!("test {}", test.iter().map(|x| one_display(x)).sum::<i64>());
    println!("{}", input.iter().map(|x| one_display(x)).sum::<i64>());
}
