use std::collections::{HashMap, HashSet};
use std::fs;
use crate::day10::part1::compute_score1;

fn compute_score2(incomplete_expr: &str) -> usize {
    // expressions should be incomplete
    let scores_by_char: HashMap<char, usize> = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    let open_chars = ['(', '[', '{', '<'];
    let close_chars = [')', ']', '}', '>'];
    let mut stack: Vec<char> = Vec::new();
    for c in incomplete_expr.chars() {
        if open_chars.contains(&c) {
            stack.push(c);
        } else if close_chars.contains(&c) {
            stack.pop();
        }
    }
    // incomplete expressions need to be autocompleted
    let mut score:usize = 0;
    while !stack.is_empty() {
        let c = stack.pop().unwrap();
        let index_of_c = open_chars.iter().position(|x| (*x) == c).unwrap();
        let opposite_c = close_chars[index_of_c];
        score = score * 5 + scores_by_char[&opposite_c];
    }
    return score;
}

pub fn solution() -> usize {
    let text = fs::read_to_string("src/day10/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim().split('\n').collect();
    let incomplete_expressions: Vec<&str> = lines.into_iter().filter(|line| compute_score1(line) == 0).collect();
    let mut scores:Vec<usize> = incomplete_expressions.into_iter().map(|expr| compute_score2(expr)).collect();
    scores.sort();
    return scores[scores.len() / 2];
}