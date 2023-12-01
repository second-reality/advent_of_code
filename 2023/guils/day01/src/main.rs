use lazy_static::lazy_static;
use maplit::hashmap;
use regex::Regex;
use std::collections::HashMap;

//const INPUT: &str = include_str!("../test.txt");
//const INPUT: &str = include_str!("../test2.txt");
const INPUT: &str = include_str!("../input.txt");
const STEP1: u32 = 55621;
const STEP2: u32 = 53592;

lazy_static! {
    static ref DIGITS_MAP: HashMap<&'static str, u32> = hashmap! {
        "one" => 1,
        "two" => 2, "three" => 3,
        "four" => 4, "five" => 5,
        "six" => 6, "seven" => 7,
        "eight" => 8, "nine" => 9,
    };
    static ref RE: Regex = replace_map_regexp(&DIGITS_MAP);
    static ref REV_RE: Regex = rev_replace_map_regexp(&DIGITS_MAP);
}

fn replace_map_regexp(map: &HashMap<&str, u32>) -> Regex {
    let re_str = map
        .keys()
        .map(|k| k.to_string())
        .collect::<Vec<_>>()
        .join("|");
    Regex::new(format!("(\\d|{})", re_str).as_str()).unwrap()
}

fn rev_replace_map_regexp(map: &HashMap<&str, u32>) -> Regex {
    let rev_re_str = map
        .keys()
        .map(|k| k.chars().rev().collect::<String>())
        .collect::<Vec<_>>()
        .join("|");
    Regex::new(format!("(\\d|{})", rev_re_str).as_str()).unwrap()
}

fn read_input() -> Vec<String> {
    INPUT.trim().split('\n').map(str::to_string).collect()
}

fn get_calibration(line: &str) -> u32 {
    let chars: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();
    chars[0].to_digit(10).unwrap() * 10 + chars[chars.len() - 1].to_digit(10).unwrap()
}

fn step1() {
    let input = read_input();
    let res: u32 = input.iter().map(|l| get_calibration(l)).sum();
    println!("step1: {res}");
    assert!(res == STEP1)
}

fn get_calibration2(line: &str) -> u32 {
    let first = RE.find(line).unwrap().as_str().to_string();
    let last = REV_RE
        .find(line.chars().rev().collect::<String>().as_str())
        .unwrap()
        .as_str()
        .chars()
        .rev()
        .collect::<String>();
    let vfirst = if let Some(&v) = DIGITS_MAP.get(first.as_str()) {
        v
    } else {
        first.parse::<u32>().unwrap()
    };
    let vlast = if let Some(&v) = DIGITS_MAP.get(last.as_str()) {
        v
    } else {
        last.parse::<u32>().unwrap()
    };
    vfirst * 10 + vlast
}

fn step2() {
    let input = read_input();
    let res: u32 = input.iter().map(|l| get_calibration2(l)).sum();
    println!("step2: {res}");
    assert!(res == STEP2);
}

fn main() {
    step1();
    step2();
}
