use itertools::Itertools;

//const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");
const KEY: i64 = 811589153;

fn read_input() -> Vec<i64> {
    INPUT
        .trim()
        .split('\n')
        .map(|x| x.parse::<i64>().unwrap())
        .collect_vec()
}

fn mix(input: &[i64], rounds: i32) -> Vec<i64> {
    let mut positions = (0..input.len()).collect_vec();
    let len = input.len() as i64;
    for _ in 0..rounds {
	for (idx, val) in input.iter().enumerate() {
	    let pos = positions.iter().position(|&x| x == idx).unwrap();
	    let target_pos = (pos as i64 + val).rem_euclid(len - 1) as usize;
	    positions.remove(pos);
	    positions.insert(target_pos, idx);
	}
    }
    positions.iter().map(|&x| input[x]).collect()
}

fn compute_sum(input: &[i64], positions: &[i64]) -> i64 {
    let pos_0 = positions
        .iter()
        .take_while(|&x| *x != 0)
        .count();
    [1000, 2000, 3000]
        .iter()
        .map(|x| positions[(pos_0 + x) % input.len()])
        .sum()
}

fn step1() {
    let input = read_input();
    let positions = mix(&input, 1);
    let res = compute_sum(&input, &positions);
    println!("step1: {res}");
}

fn step2() {
    let input = read_input();
    let input = input.iter().map(|x| x * KEY).collect_vec();
    let positions = mix(&input, 10);
    let res = compute_sum(&input, &positions);
    println!("step2: {res}");
}

fn main() {
    step1();
    step2();
}
