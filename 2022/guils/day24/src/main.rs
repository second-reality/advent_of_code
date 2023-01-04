use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

//const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");

type Coord = (i32, i32);
type Map = HashMap<Coord, char>;
type VMap = Vec<(Coord, char)>;
type VSet = HashSet<Coord>;
type TMap = HashMap<(Coord, usize), usize>;

fn read_input() -> VMap {
    INPUT
        .trim()
        .split('\n')
        .enumerate()
        .flat_map(|(l, line)| {
            line.chars().enumerate().filter_map(move |(c, ch)| {
                if ch != '.' {
                    Some(((l as i32, c as i32), ch))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn map_dim(map: &VMap) -> Coord {
    (
        map.iter().map(|(pos, _)| pos.0).max().unwrap() - 1,
        map.iter().map(|(pos, _)| pos.1).max().unwrap() - 1,
    )
}

fn update_map(map: &VMap) -> VMap {
    let (min_l, min_c) = (1, 1);
    let (max_l, max_c) = map_dim(map);
    let update_c = |c: i32| min_c + (c - min_c).rem_euclid(max_c - min_c + 1);
    let update_l = |l: i32| min_l + (l - min_l).rem_euclid(max_l - min_l + 1);
    map.iter()
        .map(|&(pos, ch)| {
            if ch == '#' {
                (pos, ch)
            } else {
                (
                    match ch {
                        '>' => (pos.0, update_c(pos.1 + 1)),
                        'v' => (update_l(pos.0 + 1), pos.1),
                        '<' => (pos.0, update_c(pos.1 - 1)),
                        '^' => (update_l(pos.0 - 1), pos.1),
                        _ => unreachable!(),
                    },
                    ch,
                )
            }
        })
        .collect()
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i32, b: i32) -> i32 {
    (a * b).abs() / gcd(a, b)
}

fn walk_step(vec_map: &Vec<VMap>, start: Coord, end: Coord, step: usize) -> usize {
    let dim = map_dim(&vec_map[0]);
    let mut time_map = TMap::new();
    let mut todo = VecDeque::<(Coord, usize)>::new();
    let mut min = usize::MAX;
    let mut sets = Vec::<VSet>::new();
    for map in vec_map.iter() {
        sets.push(map.iter().map(|&(pos, _)| pos).collect::<VSet>());
    }
    todo.push_front((start, step));
    //println!("Initial: step {step} {start:?}:");
    //print_map_pos(&vec_map[step % vec_map.len()], start);

    'stack: while !todo.is_empty() {
        let (pos, step) = todo.pop_back().unwrap();
        if step + 1 > min {
            continue 'stack;
        }
        let prev_map_id = step % vec_map.len();
        if let Some(&s) = time_map.get(&(pos, prev_map_id)) {
            if step >= s {
                continue 'stack;
            }
        }
        time_map.insert((pos, prev_map_id), step);

        let map_id = (step + 1) % vec_map.len();
        let set = &sets[map_id];

        let cands = [
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 - 1),
            pos,
        ]
        .iter()
        .filter_map(|&next| {
            if next.0 >= 1 && next.0 <= dim.0 && next.1 >= 1 && next.1 <= dim.1
                || next == end
                || next == start
            {
                Some(next)
            } else {
                None
            }
        })
        .collect_vec();
        if cands
            .iter()
            .filter_map(|&next| if next == end { Some(next) } else { None })
            .count()
            == 1
        {
            if step + 1 < min {
                min = step + 1;
            }
            continue 'stack;
        }
        for cand in cands.iter() {
            if *cand == start || !set.contains(cand) {
                todo.push_front((*cand, step + 1))
            }
        }
    }
    assert!(min != usize::MAX);
    min
}

fn get_maps(map: &VMap) -> Vec<VMap> {
    let dim = map_dim(map);
    let l = lcm(dim.0, dim.1);
    let mut current_map = map.clone();
    let mut map_vec = Vec::<VMap>::new();
    for _ in 0..l {
        map_vec.push(current_map.clone());
        current_map = update_map(&current_map);
    }
    assert!(map_str(map) == map_str(&current_map));
    assert!(map_vec.len() == l as usize);
    map_vec
}

fn walk(map: &VMap) -> usize {
    let max_l = map.iter().map(|(pos, _)| pos.0).max().unwrap() - 1;
    let max_c = map.iter().map(|(pos, _)| pos.1).max().unwrap() - 1;
    let map_vec = get_maps(map);
    let (start, end) = ((0, 1), (max_l + 1, max_c));
    walk_step(&map_vec, start, end, 0)
}

fn step1() {
    let map = read_input();
    let res = walk(&map);
    println!("step1: {res}");
}

fn walk3(map: &VMap) -> usize {
    let max_l = map.iter().map(|(pos, _)| pos.0).max().unwrap() - 1;
    let max_c = map.iter().map(|(pos, _)| pos.1).max().unwrap() - 1;
    let map_vec = get_maps(map);
    let (start, end) = ((0, 1), (max_l + 1, max_c));
    let step1 = walk_step(&map_vec, start, end, 0);
    let step2 = walk_step(&map_vec, end, start, step1);
    walk_step(&map_vec, start, end, step2)
}

fn step2() {
    let map = read_input();
    let res = walk3(&map);
    println!("step2: {res}");
}

fn main() {
    step1();
    step2();
}

#[allow(dead_code)]
fn map_str_pos(vmap: &VMap, pos: Coord) -> String {
    let dim = map_dim(vmap);
    let (min_l, min_c) = (0, 0);
    let (max_l, max_c) = (dim.0 + 1, dim.1 + 1);
    let map = vmap.iter().copied().collect::<Map>();
    (0..=(max_l - min_l))
        .map(|l| {
            (min_c..=max_c)
                .map(|c| {
                    if (l + min_l, c) == pos {
                        "E".to_string()
                    } else if let Some(ch) = map.get(&(l + min_l, c)) {
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
fn map_str(vmap: &VMap) -> String {
    map_str_pos(vmap, (-1, -1))
}

#[allow(dead_code)]
fn print_map(map: &VMap) {
    println!("{}", map_str(map));
}

#[allow(dead_code)]
fn print_map_pos(map: &VMap, pos: Coord) {
    println!("{}", map_str_pos(map, pos));
}
