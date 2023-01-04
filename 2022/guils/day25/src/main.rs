use itertools::Itertools;

//const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");

const DIGITS: [char; 5] = ['0', '1', '2', '=', '-'];

fn read_input() -> Vec<i64> {
    INPUT.trim().split('\n').map(base5_parse).collect_vec()
}

fn base5_parse(base5: &str) -> i64 {
    base5
        .chars()
        .rev()
        .fold((0, 1), |(val, base), d| {
            let mut e = DIGITS.iter().position(|&c| c == d).unwrap() as i64;
            if e > 2 {
                e -= 5;
            }
            (val + e * base, base * 5)
        })
        .0
}

fn base5_list(num: i64) -> Vec<i64> {
    assert!(num >= 0);
    let mut base5 = Vec::<i64>::new();
    let mut val = num;
    while {
        let mut e = val % 5;
        val /= 5;
        if e > 2 {
            val += 1;
            e -= 5
        }
        base5.push(e);
        val > 0
    } {}
    base5
}

fn base5_str(num: i64) -> String {
    let list = base5_list(num);
    list.iter()
        .rev()
        .map(|&x| DIGITS[if x < 0 { x + 5 } else { x } as usize].to_string())
        .join("")
}

fn step1() {
    let input = read_input();
    let res = input.iter().sum();
    let base5 = base5_str(res);
    println!("step1: {base5}");
}

fn step2() {
    println!("step2: is for free!");
}

fn main() {
    step1();
    step2();
}
