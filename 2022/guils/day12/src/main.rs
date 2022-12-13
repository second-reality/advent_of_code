use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

//const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");

type Coord = (i32, i32);
type Plane = HashMap<Coord, i32>;
type Field = (Coord, Plane);
type PlayField = (Coord, Coord, Field);
type CoordSet = HashSet<Coord>;

fn read_input() -> Vec<String> {
    INPUT.trim().split('\n').map(str::to_string).collect()
}

fn parse_field(input: &[String]) -> PlayField {
    let mut plane = Plane::new();
    let mut start: Coord = (0, 0);
    let mut end: Coord = (0, 0);
    let dim = (input.len() as i32, input[0].len() as i32);
    for (l, line) in input.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            let coord = (l as i32, c as i32);
            let h = match ch {
                'S' => {
                    start = coord;
                    0
                }
                'E' => {
                    end = coord;
                    25
                }
                l => l as i32 - 'a' as i32,
            };
            plane.insert(coord, h);
        }
    }
    (start, end, (dim, plane))
}

fn all_dists_from(playfield: &PlayField, starts: &CoordSet) -> Plane {
    let (_, _, (_, plane)) = playfield;
    let mut coords = starts.clone();
    let mut dists = Plane::from_iter(coords.iter().map(|coord| (*coord, 0)));
    while !coords.is_empty() {
        for coord in coords.clone() {
            coords.remove(&coord);
            let dist = *dists.get(&coord).unwrap() + 1;
            let nears = neighbours(plane, &coord);
            let nexts = nears
                .iter()
                .filter(|coord| match dists.get(coord) {
                    Some(x) => dist < *x,
                    None => true,
                })
                .collect_vec();
            for next in nexts {
                dists.insert(*next, dist);
                coords.insert(*next);
            }
        }
    }
    dists
}

fn all_dists_from_start(playfield: &PlayField) -> Plane {
    let (start, _, _) = playfield;
    let coords = CoordSet::from([*start]);
    all_dists_from(playfield, &coords)
}

fn neighbours(plane: &Plane, pos: &Coord) -> CoordSet {
    let h = plane.get(pos).unwrap();
    vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .filter_map(|(dx, dy)| {
            let new_pos = (pos.0 + dx, pos.1 + dy);
            match plane.get(&new_pos) {
                Some(new_h) => {
                    if new_h - h <= 1 {
                        Some(new_pos)
                    } else {
                        None
                    }
                }
                None => None,
            }
        })
        .collect()
}

fn step1() {
    let input = read_input();
    let field = parse_field(&input);
    let dists = all_dists_from_start(&field);
    let res = dists.get(&field.1).unwrap();
    println!("step1: {res}");
}

fn find_starts(playfield: &PlayField) -> CoordSet {
    let (_, _, (_, plane)) = playfield;
    plane
        .iter()
        .filter_map(|(coord, v)| if *v == 0 { Some(*coord) } else { None })
        .collect()
}

fn step2() {
    let input = read_input();
    let field = parse_field(&input);
    let starts = find_starts(&field);
    let dists = all_dists_from(&field, &starts);
    let res = dists.get(&field.1).unwrap();
    println!("step2: {res}");
}

fn main() {
    step1();
    step2();
}

#[allow(dead_code)]
fn print_field_inter(field: &PlayField, inter: &Plane) {
    let (start, end, (dim, plane)) = field;
    for l in 0..dim.0 {
        let line = (0..dim.1)
            .map(|c| {
                if (l, c) == *end {
                    "E".to_string()
                } else if (l, c) == *start {
                    "S".to_string()
                } else {
                    let base = if inter.contains_key(&(l, c)) {
                        'a'
                    } else {
                        'A'
                    };
                    let v = plane.get(&(l, c)).unwrap();
                    ((base as u8 + *v as u8) as char).to_string()
                }
            })
            .join("");
        println!("{}", line);
    }
}

#[allow(dead_code)]
fn print_field(field: &PlayField) {
    print_field_inter(field, &field.2 .1);
}
