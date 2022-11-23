use std::collections::{HashMap};
use std::fs;

pub fn compute_score1(expression:&str) -> usize{
    let scores_by_char:HashMap<char, usize> = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let open_chars = ['(', '{', '[', '<'];
    let close_chars = [')', '}', ']', '>'];
    let mut stack:Vec<char> = Vec::new();
    for c in expression.chars() {
        if open_chars.contains(&c) {
            stack.push(c);
        } else if close_chars.contains(&c)  {
            let prev_char = stack.pop();
            let index_of_c = close_chars.iter().position(|x| (*x) == c).unwrap();
            let opposite_char = open_chars[index_of_c];
            // check for illegal parenthesis (bad end parenthesis)
            if prev_char.is_none() || (prev_char.unwrap() != opposite_char) {
                return *scores_by_char.get(&c).unwrap();
            }
        }
    }
    // legal parenthesis or incomplete
    return 0;
}
pub fn solution() -> usize {
    let text = fs::read_to_string("src/day10/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim().split('\n').collect();
    return lines.into_iter().map(|line| compute_score1(line)).sum();
}