use std::collections::HashSet;

use itertools::Itertools;

fn get_line(s: &str) -> Vec<String> {
    let mut line = s.split(" | ");
    let right = line.nth(1).unwrap();
    right.split_whitespace().map(String::from).collect()
}

fn get_input(s: &str) -> Vec<Vec<String>> {
    s.lines().map(get_line).collect()
}

fn get_possible_digit(s: &str, permutation: &[&char]) -> Option<u8> {
    assert_eq!(7, permutation.len());
    let mut changed = String::from(s);
    for (index, &&val) in permutation.iter().enumerate() {
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
    for perm in segments
        .iter()
        .permutations(segments.len())
        .filter(|p| get_possible_digit(s, p).is_some())
    {
        let v: Vec<char> = perm.into_iter().copied().collect();
        working.insert(v);
    }
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

fn main() {
    let input = get_input(include_str!("../input.txt"));
    let test = get_input(include_str!("../test_input.txt"));
    println!("test {}", unique_digits(&test));
    println!("{}", unique_digits(&input));

    let test = [
        "be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb", "fabcd", "edb",
    ];
    let mut blop = possible_segments(test[0]);
    for s in test {
        blop = blop.intersection(&possible_segments(s)).cloned().collect();
        println!("From: {}: {:?}", s, blop);
    }
    assert_eq!(1, blop.len());
}
