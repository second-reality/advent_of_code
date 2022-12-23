use itertools::Itertools;
use std::collections::HashMap;

//const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");
const KEY: i64 = 811589153;
const ROUNDS: i32 = 10;

fn read_input() -> Vec<i64> {
    INPUT
        .trim()
        .split('\n')
        .map(|x| x.parse::<i64>().unwrap())
        .collect_vec()
}

fn id_map(input: &[i64]) -> HashMap<i64, i64> {
    input
        .iter()
        .enumerate()
        .map(|(i, &x)| (i as i64, x))
        .collect()
}

fn initial_positions(input: &[i64]) -> Vec<i64> {
    input
        .iter()
        .enumerate()
        .map(|(i, _)| i as i64)
        .collect_vec()
}

fn mix(input: &[i64], input_positions: &[i64]) -> Vec<i64> {
    let len = input.len() as i64;
    let mut current_pos = input_positions
        .iter()
        .enumerate()
        .map(|(i, &x)| (x, i as i64))
        .collect::<HashMap<_, _>>();
    let mut start = 0;
    let mut positions = input_positions.to_vec();
    for (i, n) in input.iter().enumerate().map(|(i, &x)| (i as i64, x)) {
        let mut pos = *current_pos.get(&i).unwrap();
        let mut delta = n.abs();
        let ds = n.signum();
        delta %= len - 1;
        while delta > 0 {
            let next = (pos + ds + len) % len;
            if next == start {
                start = (start - ds + len) % len;
            }
            let ni = positions[next as usize];
            positions[pos as usize] = ni;
            positions[next as usize] = i;
            current_pos.insert(ni, pos);
            current_pos.insert(i, next);
            pos = next;
            delta -= 1;
        }
    }
    (0..len)
        .map(|x| positions[((x + start) % len) as usize])
        .collect_vec()
}

fn compute_sum(input: &[i64], positions: &[i64]) -> i64 {
    let id_map = id_map(input);
    let pos_0 = positions
        .iter()
        .take_while(|x| *id_map.get(x).unwrap() != 0)
        .count();
    [1000, 2000, 3000]
        .iter()
        .map(|x| positions[(pos_0 + x) % input.len()])
        .map(|x| id_map.get(&x).unwrap())
        .sum()
}

fn compute_1(input: &[i64]) -> i64 {
    let initial = initial_positions(input);
    let positions = mix(input, &initial);
    compute_sum(input, &positions)
}

fn step1() {
    let input = read_input();
    let res = compute_1(&input);
    println!("step1: {res}");
}

fn compute_2(input: &[i64]) -> i64 {
    let input = input.iter().map(|x| x * KEY).collect_vec();
    let mut positions = initial_positions(&input);
    for _ in 1..=ROUNDS {
        positions = mix(&input, &positions);
    }
    compute_sum(&input, &positions)
}

fn step2() {
    let input = read_input();
    let res = compute_2(&input);
    println!("step2: {res}");
}

fn main() {
    step1();
    step2();
}
