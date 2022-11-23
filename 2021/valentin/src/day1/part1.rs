use std::fs;

pub fn solution() -> usize {
    let text = fs::read_to_string("src/day1/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim().split('\n').collect();
    let numbers: Vec<i32> = lines.into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
    let count_increase = (1..numbers.len()).into_iter().reduce(|acc, i| {
        if numbers[i] - numbers[i - 1] > 0 { acc + 1 } else { acc }
    });
    return count_increase.expect("error");
}