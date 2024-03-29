use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

// const INPUT: &str = include_str!("../test.txt");
// const STEP1: u32 = 4361;
// const STEP2: u32 = 467835;
const INPUT: &str = include_str!("../input.txt");
const STEP1: u32 = 553825;
const STEP2: u32 = 93994191;

type Coord = (i32, i32);
type PartId = (char, usize);
type Plane = HashMap<Coord, PartId>;
type Parts = Vec<String>;
type Graph = HashMap<PartId, HashSet<PartId>>;

lazy_static! {
    static ref BALL: Vec<Coord> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1)
    ];
}

fn read_input() -> Vec<String> {
    INPUT.trim().split('\n').map(str::to_string).collect()
}

fn get_plane(lines: &[String]) -> (Plane, Parts) {
    let mut map = Plane::new();
    let mut parts = Parts::new();
    let mut part_num: usize = 0;
    for (i, line) in lines.iter().enumerate() {
        let mut last_is_digit = false;
        for (j, c) in line.chars().enumerate() {
            let coord = (i as i32, j as i32);
            if c.is_ascii_digit() {
                if !last_is_digit {
                    map.insert(coord, ('P', part_num));
                    part_num += 1;
                    parts.push(c.to_string());
                } else {
                    map.insert(coord, ('P', part_num - 1));
                    let last = parts.pop().unwrap();
                    parts.push(last + &c.to_string());
                }
                last_is_digit = true;
            } else {
                if c != '.' {
                    map.insert(coord, ('C', part_num));
                    part_num += 1;
                    parts.push(c.to_string());
                }
                last_is_digit = false;
            }
        }
    }
    (map, parts)
}

fn get_graph(plane: &Plane) -> Graph {
    let mut graph = Graph::new();
    for ((i, j), part) in plane.iter() {
        let mut set = HashSet::<PartId>::new();
        for (x, y) in BALL.iter().map(|(x, y)| (i + x, j + y)) {
            if let Some(part2) = plane.get(&(x, y)) {
                if part.0 != part2.0 {
                    set.insert(*part2);
                }
            }
        }
        if let Some(oldset) = graph.get(part) {
            set.extend(oldset);
        }
        if !set.is_empty() {
            graph.insert(*part, set);
        }
    }
    graph
}

fn parts_sum(graph: &Graph, parts: &Parts) -> u32 {
    graph
        .keys()
        .filter(|(k, _)| *k == 'P')
        .fold(0, |a, (_, id)| a + parts[*id].parse::<u32>().unwrap())
}

fn step1() {
    let input = read_input();
    let (plane, parts) = get_plane(&input);
    let graph = get_graph(&plane);
    let res = parts_sum(&graph, &parts);
    println!("step1: {res}");
    assert!(res == STEP1);
}

fn gears_power(graph: &Graph, parts: &Parts) -> u32 {
    graph.iter().fold(0, |a, ((_, id), set)| {
        if parts[*id] == "*" && set.len() == 2 {
            let power = set
                .iter()
                .fold(1, |a, (_, pid)| a * parts[*pid].parse::<u32>().unwrap());
            a + power
        } else {
            a
        }
    })
}

fn step2() {
    let input = read_input();
    let (plane, parts) = get_plane(&input);
    let graph = get_graph(&plane);
    let res = gears_power(&graph, &parts);
    println!("step2: {res}");
    assert!(res == STEP2);
}

fn main() {
    step1();
    step2();
}
