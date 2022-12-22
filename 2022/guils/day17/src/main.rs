use itertools::Itertools;
use std::cmp;
use std::collections::HashMap;

//const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");
const MAX_ROCKS_1: i64 = 2022;
const MAX_ROCKS_2: i64 = 1000000000000;

type Coord = (i64, i64);
type Shape = Vec<Coord>;
type Rock = (Coord, Shape);
type Plane = HashMap<Coord, char>;
type Jet = char;
type Cache = (i64, i64, Vec<i64>);
type History = Vec<i64>;
type UniqueId = (i64, i64, String);
type UniqueMap = HashMap<UniqueId, i64>;

fn read_input() -> Vec<Jet> {
    INPUT.trim().chars().collect()
}

fn get_rocks() -> Vec<Rock> {
    vec![
        ((4, 1), vec![(0, 0), (1, 0), (2, 0), (3, 0)]),
        ((3, 3), vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]),
        ((3, 3), vec![(2, 0), (2, 1), (0, 2), (1, 2), (2, 2)]),
        ((1, 4), vec![(0, 0), (0, 1), (0, 2), (0, 3)]),
        ((2, 2), vec![(0, 0), (1, 0), (0, 1), (1, 1)]),
    ]
}

fn intersect(plane: &Plane, shape: &Shape, pos: &Coord) -> bool {
    for elt in shape.iter() {
        let pt = (pos.0 + elt.0, pos.1 - elt.1);
        if plane.contains_key(&pt) {
            return true;
        }
    }
    false
}

fn set_point(plane: &mut Plane, pos: &Coord, ch: char, cache: &mut Cache) {
    if pos.1 > cache.2[pos.0 as usize] {
        cache.2[pos.0 as usize] = pos.1;
    }
    plane.insert(*pos, ch);
}

fn set_at_rest(plane: &mut Plane, shape: &Shape, pos: &Coord, cache: &mut Cache) {
    for pt in shape.iter().map(|elt| (pos.0 + elt.0, pos.1 - elt.1)) {
        set_point(plane, &pt, '#', cache);
    }
}

fn update_cache(plane: &mut Plane, cache: &mut Cache, jet_idx: i64, rock_idx: i64) -> UniqueId {
    let min = *cache.2.iter().min().unwrap();
    let max = *cache.2.iter().max().unwrap();
    cache.0 = cmp::max(cache.0, min);
    let mut chars = Vec::<char>::new();
    for y in cache.0..=max {
        for x in 0..cache.1 {
            if plane.contains_key(&(x, y)) {
                chars.push('#')
            } else {
                chars.push(' ')
            }
        }
    }
    let field_id = chars.iter().collect::<String>();
    (jet_idx, rock_idx, field_id)
}

fn get_cache(width: i64) -> Cache {
    let mut cache = (-1, width, Vec::new());
    for _ in 0..width {
        cache.2.push(-1)
    }
    cache
}

fn floor(plane: &mut Plane, width: i64, y: i64) {
    for x in -1..=width {
        plane.insert((x, y), '-');
    }
}

fn falling_loop(rocks: &[Rock], jets: &[Jet], max_rocks: i64) -> i64 {
    let (left_s, bottom_s) = (2, 3);
    let width = 7;
    let mut n_rocks = 0;
    let mut rock_idx = 0;
    let mut jet_idx = 0;
    let mut plane = Plane::new();
    let mut height = 0;
    let mut cache = get_cache(width);
    let mut history = History::new();
    let mut unique_map = UniqueMap::new();
    let cached = true;
    let find_cycle = true;

    if !cached {
        floor(&mut plane, width, -1)
    }

    loop {
        let (size, shape) = &rocks[rock_idx];
        rock_idx = (rock_idx + 1) % rocks.len();
        let mut pos = (left_s, height + bottom_s + size.1 - 1);
        loop {
            let jet = &jets[jet_idx];
            jet_idx = (jet_idx + 1) % jets.len();
            let next_pos = (
                if *jet == '<' {
                    cmp::max(0, pos.0 - 1)
                } else {
                    cmp::min(pos.0 + 1, width - size.0)
                },
                pos.1,
            );
            if !intersect(&plane, shape, &next_pos) {
                pos = next_pos;
            }
            let next_pos = (pos.0, pos.1 - 1);
            if next_pos.1 - size.1 + 1 < 0 || intersect(&plane, shape, &next_pos) {
                break;
            }
            pos = next_pos;
        }
        set_at_rest(&mut plane, shape, &pos, &mut cache);
        n_rocks += 1;
        height = cmp::max(height, pos.1 + 1);
        history.push(height);
        if n_rocks == max_rocks {
            break;
        }
        if cached {
            let id = update_cache(&mut plane, &mut cache, jet_idx as i64, rock_idx as i64);
            if find_cycle {
                if let Some(r) = unique_map.get(&id) {
                    let delta_r = n_rocks - r;
                    let (div, rem) = (
                        (max_rocks - n_rocks) / delta_r,
                        (max_rocks - n_rocks) % delta_r,
                    );
                    let delta_height = (height - history[*r as usize - 1]) * div
                        + history[*r as usize + rem as usize - 1]
                        - history[*r as usize - 1];
                    height += delta_height;
                    break;
                } else {
                    unique_map.insert(id, n_rocks);
                }
            }
        }
    }
    height
}

fn step1() {
    let jets = read_input();
    let rocks = get_rocks();
    let height = falling_loop(&rocks, &jets, MAX_ROCKS_1);
    println!("step1: {height}");
}

fn step2() {
    let jets = read_input();
    let rocks = get_rocks();
    let height = falling_loop(&rocks, &jets, MAX_ROCKS_2);
    println!("step2: {height}");
}

fn main() {
    step1();
    step2();
}

#[allow(dead_code)]
fn print_plane(plane: &Plane) {
    let min_x = plane.keys().map(|c| c.0).min().unwrap();
    let max_x = plane.keys().map(|c| c.0).max().unwrap();
    let min_y = plane.keys().map(|c| c.1).min().unwrap();
    let max_y = plane.keys().map(|c| c.1).max().unwrap();
    for y in 0..=(max_y - min_y) {
        let line = (min_x..=max_x)
            .map(|x| {
                if let Some(c) = plane.get(&(x, max_y - y)) {
                    c.to_string()
                } else {
                    ".".to_string()
                }
            })
            .join("");
        println!("{}", line);
    }
}
