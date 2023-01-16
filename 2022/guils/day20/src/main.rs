use itertools::Itertools;
use tree::*;

//const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");
const KEY: i64 = 811589153;
const USE_TREE: bool = true;
//const USE_TREE: bool = false;

fn read_input() -> Vec<i64> {
    INPUT
        .trim()
        .split('\n')
        .map(|x| x.parse::<i64>().unwrap())
        .collect_vec()
}

fn mix_vec(input: &[i64], rounds: i32) -> Vec<i64> {
    let positions_index =
        |positions: &[usize], idx: usize| positions.iter().position(|&x| x == idx).unwrap();
    let mut positions = (0..input.len()).collect_vec();
    let len = input.len() as i64;
    for _r in 0..rounds {
        for (idx, val) in input.iter().enumerate() {
            let pos = positions_index(&positions, idx);
            let target_pos = (pos as i64 + val).rem_euclid(len - 1) as usize;
            positions.remove(pos);
            positions.insert(target_pos, idx);
        }
    }
    positions.iter().map(|&x| input[x]).collect()
}

fn mix_tree(input: &[i64], rounds: i32) -> Vec<i64> {
    let mut tree = input.iter().copied().collect::<OrdTree<_>>();
    let len = input.len() as i64;
    for _ in 0..rounds {
        for (idx, val) in input.iter().enumerate() {
            let pos = tree.index_cell(idx);
            let target_pos = (pos as i64 + val).rem_euclid(len - 1) as usize;
            tree.remove_at_cell(idx);
            tree.insert_cell(target_pos, idx);
        }
    }
    tree.to_vec()
}

fn mix(input: &[i64], rounds: i32) -> Vec<i64> {
    if USE_TREE {
        mix_tree(input, rounds)
    } else {
        mix_vec(input, rounds)
    }
}

fn compute_sum(input: &[i64], positions: &[i64]) -> i64 {
    let pos_0 = positions.iter().take_while(|&x| *x != 0).count();
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
