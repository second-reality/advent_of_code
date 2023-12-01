use lazy_static::lazy_static;
use maplit::hashmap;
use regex::{Captures, Regex};
use std::collections::HashMap;

//const INPUT: &str = include_str!("../test.txt");
//const INPUT: &str = include_str!("../test2.txt");
const INPUT: &str = include_str!("../input.txt");
const STEP1: u32 = 55621;
const STEP2: u32 = 53592;

lazy_static! {
    static ref DIGITS_MAP: HashMap<&'static str, &'static str> = hashmap! {
        "one" => "1",
        "two" => "2", "three" => "3",
        "four" => "4", "five" => "5",
        "six" => "6", "seven" => "7",
        "eight" => "8", "nine" => "9",
    };
}

fn read_input() -> Vec<String> {
    INPUT.trim().split('\n').map(str::to_string).collect()
}

fn get_calibration(line: &str) -> u32 {
    let chars: Vec<char> = line
        .chars()
        .filter(|c| c.is_ascii_digit() && *c != '0')
        .collect();
    chars[0].to_digit(10).unwrap() * 10 + chars[chars.len() - 1].to_digit(10).unwrap()
}

fn step1() {
    let input = read_input();
    let res: u32 = input.iter().map(|l| get_calibration(l)).sum();
    println!("step1: {res}");
    assert!(res == STEP1)
}

fn replace_digits(lines: &[String]) -> (Vec<String>, Vec<String>) {
    let re = replace_map_regexp(&DIGITS_MAP);
    let rev_re = rev_replace_map_regexp(&DIGITS_MAP);
    let strs: Vec<String> = lines
        .iter()
        .map(|l| {
            re.replace(l, |caps: &Captures| DIGITS_MAP[&caps[0]])
                .to_string()
        })
        .collect();
    let rev_strs: Vec<String> = lines
        .iter()
        .map(|l| {
            rev_re
                .replace(&l.chars().rev().collect::<String>(), |caps: &Captures| {
                    DIGITS_MAP[&caps[0].chars().rev().collect::<String>().as_str()]
                })
                .to_string()
        })
        .collect();
    (strs, rev_strs)
}

fn replace_map_regexp(map: &HashMap<&str, &str>) -> Regex {
    let re_str = map
        .keys()
        .map(|k| k.to_string())
        .collect::<Vec<_>>()
        .join("|");
    Regex::new(format!("({})", re_str).as_str()).unwrap()
}

fn rev_replace_map_regexp(map: &HashMap<&str, &str>) -> Regex {
    let rev_re_str = map
        .keys()
        .map(|k| k.chars().rev().collect::<String>())
        .collect::<Vec<_>>()
        .join("|");
    Regex::new(format!("({})", rev_re_str).as_str()).unwrap()
}

fn get_first_nz_digit(line: &str) -> u32 {
    line.chars()
        .find(|c| c.is_ascii_digit() && *c != '0')
        .unwrap()
        .to_digit(10)
        .unwrap()
}

fn get_calibration_sum(input1: &[String], input2: &[String]) -> u32 {
    input1
        .iter()
        .zip(input2.iter())
        .map(|(a, b)| get_first_nz_digit(a) * 10 + get_first_nz_digit(b))
        .sum()
}

fn step2() {
    let input = read_input();
    let (input2, input3) = replace_digits(&input);
    let res: u32 = get_calibration_sum(&input2, &input3);
    println!("step2: {res}");
    assert!(res == STEP2);
}

fn main() {
    step1();
    step2();
}
