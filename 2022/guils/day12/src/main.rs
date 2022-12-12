use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

//const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");

type Coord = (i32, i32);
type Height = i32;
type Plane = HashMap<Coord, Height>;
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
    input.iter().enumerate().for_each(|(l, line)| {
        line.chars().enumerate().for_each(|(c, ch)| {
            let coord = (l as i32, c as i32);
            plane.insert(
                coord,
                match ch {
                    'S' => {
                        start = coord;
                        0
                    }
                    'E' => {
                        end = coord;
                        25
                    }
                    x => x as i32 - 'a' as i32,
                },
            );
        })
    });
    (start, end, (dim, plane))
}

fn all_dists_from(plane: &Plane, starts: &CoordSet) -> Plane {
    let mut coords = starts.clone();
    let mut dists = Plane::from_iter(coords.iter().map(|coord| (*coord, 0)));
    while !coords.is_empty() {
        let mut new_coords = coords.clone();
        for coord in coords {
            new_coords.remove(&coord);
            let d = *dists.get(&coord).unwrap();
            let local_coords = search_coords(plane, &coord);
            for next_coord in local_coords {
                let next_d = dists.get(&next_coord);
                let new_d = match next_d {
                    Some(x) => {
                        if d + 1 < *x {
                            d + 1
                        } else {
                            0
                        }
                    }
                    None => d + 1,
                };
                if new_d > 0 {
                    dists.insert(next_coord, new_d);
                    new_coords.insert(next_coord);
                }
            }
        }
        coords = new_coords;
    }
    dists
}

fn all_dists(playfield: &PlayField) -> Plane {
    let (start, _, (_, plane)) = playfield;
    let coords: CoordSet = vec![*start].iter().copied().collect();
    all_dists_from(plane, &coords)
}

fn search_coords(plane: &Plane, pos: &Coord) -> CoordSet {
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
    let dists = all_dists(&field);
    let res = dists.get(&field.1).unwrap();
    println!("step1: {res}");
}

fn find_starts(plane: &Plane) -> CoordSet {
    plane
        .iter()
        .filter_map(|(coord, v)| if *v == 0 { Some(coord) } else { None })
        .copied()
        .collect()
}

fn step2() {
    let input = read_input();
    let (_, end, (_, plane)) = parse_field(&input);
    let starts = find_starts(&plane);
    let dists = all_dists_from(&plane, &starts);
    let res = dists.get(&end).unwrap();
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
