use itertools::Itertools;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

//const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");
const DEBUG: bool = false;

type Coord = (i32, i32);
type Map = HashMap<Coord, char>;
type VMap = Vec<(Coord, char)>;
type VSet = Vec<bool>;

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

fn get_maps(map: &VMap) -> Vec<VSet> {
    let dim = map_dim(map);
    let l = lcm(dim.0, dim.1);
    let mut current_map = map.clone();
    let mut maps = Vec::<VSet>::new();
    for _ in 0..l {
        let mut map = vec![false; (dim.0 + 2) as usize * (dim.1 + 2) as usize];
        for &((l, c), _) in current_map.iter() {
            map[l as usize * (dim.1 + 2) as usize + c as usize] = true;
        }
        maps.push(map.to_vec());
        current_map = update_map(&current_map);
    }
    maps
}

fn walk(maps: &[VSet], dim: Coord, start: Coord, end: Coord, init_step: i32) -> i32 {
    let mut visited = HashSet::<(Coord, usize)>::new();
    let mut todo = BinaryHeap::new();
    let mut moves = 0;
    let mut skip_map = 0;
    let dist_heuristic =
        |pos: Coord, step: i32| step + (end.0 - pos.0).abs() + (end.1 - pos.1).abs();
    todo.push((-dist_heuristic(start, init_step), start, init_step));
    while let Some((_, pos, step)) = todo.pop() {
        if pos == end {
            if DEBUG {
                println!("moves: {moves}, skip_map {skip_map}");
            }
            return step;
        }
        moves += 1;
        if DEBUG && moves % 50000 == 0 {
            println!(
                "step: {step}: moves: {moves}, stack: {}, skip_map {skip_map}",
                todo.len()
            );
        }
        let map_id = (step as usize + 1) % maps.len();
        let map = &maps[map_id];
        for next in [
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 - 1),
            pos,
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 + 1),
        ] {
            if !(next.0 >= 1
                && next.0 <= dim.0
                && next.1 >= 1
                && next.1 <= dim.1
                && !map[next.0 as usize * (dim.1 + 2) as usize + next.1 as usize]
                || next == end
                || next == start)
            {
                continue;
            }
            if visited.contains(&(next, map_id)) {
                skip_map += 1;
                continue;
            }
            visited.insert((next, map_id));
            todo.push((-dist_heuristic(next, step + 1), next, step + 1));
        }
    }
    unreachable!()
}

fn step1(maps: &[VSet], dim: Coord) {
    let (start, end) = ((0, 1), (dim.0 + 1, dim.1));
    let res = walk(maps, dim, start, end, 0);
    println!("step1: {res}");
}

fn step2(maps: &[VSet], dim: Coord) {
    let (start, end) = ((0, 1), (dim.0 + 1, dim.1));
    let step1 = walk(maps, dim, start, end, 0);
    let step2 = walk(maps, dim, end, start, step1);
    let res = walk(maps, dim, start, end, step2);
    println!("step2: {res}");
}

fn main() {
    let map = read_input();
    let maps = get_maps(&map);
    let dim = map_dim(&map);
    step1(&maps, dim);
    step2(&maps, dim);
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
