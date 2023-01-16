use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

//const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");

type Coord = (i32, i32);
type Map = HashMap<Coord, char>;
type Set = HashSet<Coord>;
type MapCount = HashMap<Coord, i32>;

const DIRS: [Coord; 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)]; // N, E, S, W

fn read_input() -> Map {
    INPUT
        .trim()
        .split('\n')
        .enumerate()
        .flat_map(|(l, line)| {
            line.chars().enumerate().filter_map(move |(c, ch)| {
                if ch == '#' {
                    Some(((l as i32, c as i32), ch))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn vec_add(a: Coord, b: Coord) -> Coord {
    (a.0 + b.0, a.1 + b.1)
}

fn get_all_dirs() -> Vec<Coord> {
    [
        DIRS[0],                   // N
        vec_add(DIRS[0], DIRS[1]), // NE
        DIRS[1],                   // E
        vec_add(DIRS[1], DIRS[2]), // SE
        DIRS[2],                   // S
        vec_add(DIRS[2], DIRS[3]), // SW
        DIRS[3],                   // W
        vec_add(DIRS[3], DIRS[0]), // NW
    ]
    .into_iter()
    .collect()
}

fn set_count_empty(map: &Set) -> i32 {
    let cols = || map.iter().map(|(_, c)| *c);
    let lines = || map.iter().map(|(l, _)| *l);
    let min_c = cols().min().unwrap();
    let max_c = cols().max().unwrap();
    let min_l = lines().min().unwrap();
    let max_l = lines().max().unwrap();
    let size = (max_c - min_c + 1) * (max_l - min_l + 1);
    size - map.len() as i32
}

fn rounds(map: &Map, max: i32) -> (Set, i32) {
    let all_dirs = get_all_dirs();
    let dirs = [0, 1, 7, 4, 3, 5, 6, 7, 5, 2, 1, 3];
    let mut dir = 0;
    let mut set = map.keys().copied().collect::<Set>();
    let mut rounds = 0;

    loop {
        let mut elts = Vec::<(Coord, Coord)>::new();
        let mut count = MapCount::new();
        'a: for &pos in set.iter() {
            let around = all_dirs
                .iter()
                .filter_map(|&d| set.get(&vec_add(pos, d)))
                .count();
            if around == 0 {
                *count.entry(pos).or_default() += 1;
                continue;
            }
            for step in 0..4 {
                let d = (dir + step * 3) % dirs.len();
                let around = dirs[d..d + 3]
                    .iter()
                    .filter_map(|&d| set.get(&vec_add(pos, all_dirs[d])))
                    .count();
                if around == 0 {
                    let new = vec_add(pos, all_dirs[dirs[d]]);
                    *count.entry(new).or_default() += 1;
                    elts.push((pos, new));
                    continue 'a;
                }
            }
            *count.entry(pos).or_default() += 1;
        }
        let mut moved = 0;
        for &(pos, new) in elts.iter() {
            let c = *count.get(&new).unwrap();
            let new_pos = if c > 1 { pos } else { new };
            if pos != new_pos {
                moved += 1;
                set.remove(&pos);
                set.insert(new);
            }
        }
        rounds += 1;
        if moved == 0 {
            break;
        }
        if rounds == max {
            break;
        }
        dir = (dir + 3) % dirs.len()
    }
    (set, rounds)
}

fn step1() {
    let input = read_input();
    let (set, _) = rounds(&input, 10);
    let res = set_count_empty(&set);
    println!("step1: {res}");
}

fn step2() {
    let input = read_input();
    let (_, round) = rounds(&input, 0);
    println!("step2: {round}");
}

fn main() {
    step1();
    step2();
}

#[allow(dead_code)]
fn map_str(map: &Map) -> String {
    let min_l = map.keys().map(|c| c.0).min().unwrap();
    let max_l = map.keys().map(|c| c.0).max().unwrap();
    let min_c = map.keys().map(|c| c.1).min().unwrap();
    let max_c = map.keys().map(|c| c.1).max().unwrap();
    (0..=(max_l - min_l))
        .map(|l| {
            (min_c..=max_c)
                .map(|c| {
                    if let Some(ch) = map.get(&(l + min_l, c)) {
                        ch.to_string()
                    } else {
                        ".".to_string()
                    }
                })
                .join("")
        })
        .join("\n")
}

#[allow(dead_code)]
fn print_map(map: &Map) {
    println!("{}", map_str(map));
}
