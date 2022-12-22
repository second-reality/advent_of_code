use itertools::Itertools;
//use std::collections::HashMap;
use std::cmp;
use std::collections::HashSet;

//const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");

type Coord = (i32, i32, i32);
fn read_input() -> Vec<Coord> {
    INPUT
        .trim()
        .split('\n')
        .map(|x| {
            x.split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec()
}

fn n_adjacent_set(set: &HashSet<Coord>, x: &Coord) -> i32 {
    let mut n = 0;
    for adj in [
        (x.0 - 1, x.1, x.2),
        (x.0 + 1, x.1, x.2),
        (x.0, x.1 - 1, x.2),
        (x.0, x.1 + 1, x.2),
        (x.0, x.1, x.2 - 1),
        (x.0, x.1, x.2 + 1),
    ] {
        if set.contains(&adj) {
            n += 1;
        }
    }
    n
}

fn surface(input: &[Coord]) -> i32 {
    let mut set = HashSet::<Coord>::new();
    let mut surface = 0;
    for x in input {
        surface += 6;
        surface -= n_adjacent_set(&set, x) * 2;
        set.insert(*x);
    }
    surface
}

fn step1() {
    let input = read_input();
    let res = surface(&input);
    println!("step1: {res}");
}

fn get_full(input: &[Coord]) -> Vec<Coord> {
    let (min_x, max_x) = input.iter().fold((i32::MAX, i32::MIN), |s, x| {
        (cmp::min(s.0, x.0), cmp::max(s.1, x.0))
    });
    let (min_y, max_y) = input.iter().fold((i32::MAX, i32::MIN), |s, x| {
        (cmp::min(s.0, x.1), cmp::max(s.1, x.1))
    });
    let (min_z, max_z) = input.iter().fold((i32::MAX, i32::MIN), |s, x| {
        (cmp::min(s.0, x.2), cmp::max(s.1, x.2))
    });
    let mut full = Vec::<Coord>::new();
    for x in min_x - 1..=max_x + 1 {
        for y in min_y - 1..=max_y + 1 {
            for z in min_z - 1..=max_z + 1 {
                full.push((x, y, z));
            }
        }
    }
    full
}

fn get_filled(input: &[Coord]) -> Vec<Coord> {
    let mut filled = input.iter().cloned().collect::<HashSet<_>>();
    let (min_x, max_x) = input.iter().fold((i32::MAX, i32::MIN), |s, x| {
        (cmp::min(s.0, x.0), cmp::max(s.1, x.0))
    });
    let (min_y, max_y) = input.iter().fold((i32::MAX, i32::MIN), |s, x| {
        (cmp::min(s.0, x.1), cmp::max(s.1, x.1))
    });
    let (min_z, max_z) = input.iter().fold((i32::MAX, i32::MIN), |s, x| {
        (cmp::min(s.0, x.2), cmp::max(s.1, x.2))
    });
    let mut currents = Vec::<Coord>::new();
    currents.push((min_x - 1, min_y - 1, min_z - 1));
    while !currents.is_empty() {
        let x = currents.pop().unwrap();
        if !filled.contains(&x) {
            filled.insert(x);
            for adj in [
                (x.0 - 1, x.1, x.2),
                (x.0 + 1, x.1, x.2),
                (x.0, x.1 - 1, x.2),
                (x.0, x.1 + 1, x.2),
                (x.0, x.1, x.2 - 1),
                (x.0, x.1, x.2 + 1),
            ] {
                if adj.0 >= min_x - 1
                    && adj.0 <= max_x + 1
                    && adj.1 >= min_y - 1
                    && adj.1 <= max_y + 1
                    && adj.2 >= min_z - 1
                    && adj.2 <= max_z + 1
                {
                    currents.push(adj);
                }
            }
        }
    }
    filled.into_iter().collect_vec()
}

fn out_surface(input: &[Coord]) -> i32 {
    let s_all = surface(input);
    let filled = get_filled(input);
    let s_filled = surface(&filled);
    let full = get_full(input);
    let s_full = surface(&full);
    s_all - (s_filled - s_full)
}

fn step2() {
    let input = read_input();
    let res = out_surface(&input);
    println!("step2: {res}");
}

fn main() {
    step1();
    step2();
}
