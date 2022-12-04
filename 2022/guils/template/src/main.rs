const INPUT: &str = include_str!("../test.txt");
//const INPUT: &str = include_str!("../input.txt");

fn read_input() -> Vec<String> {
    INPUT.trim().split('\n').map(str::to_string).collect()
}

fn step1() {
    let input = read_input();
    let res = input.len();
    println!("step1: {res}");
}

fn step2() {
    let input = read_input();
    let res = input.len();
    println!("step2: {res}");
}

fn main() {
    step1();
    step2();
}
