use std::collections::{HashMap, HashSet};
use std::fs;
use crate::helper::sort_srt;

fn custom_contains(big_string:&str, small_string:&str) -> bool {
    return small_string.chars().all(|c| big_string.contains(c));
}

fn deduce_digit_from_all_str(sorted_strings:Vec<&str>) -> HashMap<String, char> {
    assert_eq!(sorted_strings.len(), 10);
    let mut res:HashMap<String, char> = HashMap::new();
    // as input is sorted by len we can find easily obvious digits 1, 7, 4, 8
    let one = sorted_strings[0]; // 1 is len 2 min
    let seven = sorted_strings[1]; // 7 is len 3
    let four = sorted_strings[2]; // 4 is len 4
    // 3, 5, 2 are len 5
    // 0, 6, 9 are len 6
    let eight = sorted_strings[9]; // 8 is len 7 max
    res.insert(String::from(one), '1');
    res.insert(String::from(seven), '7');
    res.insert(String::from(four), '4');
    res.insert(String::from(eight), '8');

    let mut nine = "-1";
    // solve len 6 digits 9, 6, 0
    for i in 6..9 {
        let cur_char = if custom_contains(sorted_strings[i], four) {
            nine = sorted_strings[i];
            '9'
        } else if !(custom_contains(sorted_strings[i], one)) {
            '6'
        } else {
            '0'
        };
        res.insert(String::from(sorted_strings[i]), cur_char);
    }
    assert_ne!("-1", nine);
    // solve len 5 digits 3, 5, 2
    for i in 3..6 {
        let cur_char = if custom_contains(sorted_strings[i], one) {
            '3'
        } else if custom_contains(nine, sorted_strings[i]){
            '5'
        } else {
            '2'
        };
        res.insert(String::from(sorted_strings[i]), cur_char);
    }

    return res;
}

pub fn solution() -> usize {
    let text = fs::read_to_string("src/day8/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim().split('\n').collect();

    let mut sum:usize = 0;
    for line in lines {
        let split:Vec<&str> = line.split(" | ").collect();
        let mut input:Vec<String> = split[0].split(" ").map(|s| sort_srt(s)).collect();
        input.sort_by(|a, b| a.len().cmp(&b.len()));
        let input:Vec<&str> = input.iter().map(|s| s.as_str()).collect();

        let output:Vec<String> = split[1].split(" ").map(|s| sort_srt(s)).collect();

        let deduction_table = deduce_digit_from_all_str(input);


        let int4digits:String = output.into_iter().map(|s| deduction_table[&s]).collect();
        sum += int4digits.parse::<usize>().expect("error during parsing int");
    }
    return sum;
}