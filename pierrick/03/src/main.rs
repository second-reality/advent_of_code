use std::collections::HashMap;

fn get_scores(input: &[String]) -> Vec<HashMap<char, i32>> {
    let mut scores: Vec<HashMap<char, i32>> = vec![];

    for l in input {
        assert!(scores.is_empty() || l.len() == scores.len());
        scores.resize(l.len(), HashMap::new());

        for (idx, c) in l.chars().enumerate() {
            *scores[idx].entry(c).or_insert(0) += 1;
        }
    }

    scores
}

fn get_score(c: char, score: &HashMap<char, i32>) -> i32 {
    *score.get(&c).unwrap_or(&0)
}

fn get_most_frequent_char(score: &HashMap<char, i32>) -> char {
    if get_score('1', score) >= get_score('0', score) {
        '1'
    } else {
        '0'
    }
}

fn get_least_frequent_char(score: &HashMap<char, i32>) -> char {
    if get_score('1', score) >= get_score('0', score) {
        '0'
    } else {
        '1'
    }
}

fn str_bit_to_i32(s: String) -> i32 {
    let mut res = 0;
    for c in s.chars() {
        res <<= 1;
        if c == '1' {
            res += 1;
        }
    }
    res
}

fn part1(scores: &[HashMap<char, i32>]) -> i32 {
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for s in scores {
        gamma.push(get_most_frequent_char(s));
        epsilon.push(get_least_frequent_char(s));
    }

    str_bit_to_i32(gamma) * str_bit_to_i32(epsilon)
}

fn find_unique_entry(
    idx: usize,
    input: &[String],
    select_char: fn(&HashMap<char, i32>) -> char,
) -> String {
    assert!(!input.is_empty());
    if input.len() == 1 {
        return input[0].clone();
    }

    let scores = get_scores(input);
    let c = select_char(&scores[idx]);
    let selected: Vec<String> = input
        .iter()
        .filter(|s| s.chars().nth(idx).unwrap() == c)
        .cloned()
        .collect();
    find_unique_entry(idx + 1, &selected, select_char)
}

fn part2(input: &[String]) -> i32 {
    let oxygen = find_unique_entry(0, input, |s| get_most_frequent_char(s));
    let co2 = find_unique_entry(0, input, |s| get_least_frequent_char(s));
    str_bit_to_i32(oxygen) * str_bit_to_i32(co2)
}

fn get_input() -> Vec<String> {
    include_str!("../input.txt")
        .lines()
        .map(String::from)
        .collect()
}

fn main() {
    let input = get_input();
    let scores = get_scores(&input);
    println!("{}", part1(&scores));
    println!("{}", part2(&input));
}
