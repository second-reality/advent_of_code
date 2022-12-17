use itertools::Itertools;
use std::collections::HashMap;

//const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");

type Coord = (i32, i32);
type Plane = HashMap<Coord, char>;

fn read_input() -> Vec<String> {
    INPUT.trim().split('\n').map(str::to_string).collect()
}

fn parse_coords(line: &str) -> Vec<Coord> {
    line.split(" -> ")
        .map(|x| {
            x.split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec()
}

fn set_plane_wall(plane: &mut Plane, coords: &[Coord]) {
    coords.iter().fold(None, |s: Option<Coord>, (x, y)| {
        if let Some((ox, oy)) = s {
            let (dx, dy) = ((x - ox), (y - oy));
            for sx in 0..=dx.abs() {
                for sy in 0..=dy.abs() {
                    plane.insert((ox + sx * dx.signum(), oy + sy * dy.signum()), '#');
                }
            }
        }
        Some((*x, *y))
    });
}

fn parse_plane(input: &[String]) -> Plane {
    let mut plane = Plane::new();
    for line in input.iter() {
        set_plane_wall(&mut plane, &parse_coords(line))
    }
    plane
}

fn fill_plane(plane: &mut Plane, start: &Coord, floor: i32) {
    let max_y = plane.keys().map(|c| c.1).max().unwrap();
    assert!(plane.get(start).is_none());
    loop {
        let mut rest = false;
        let mut xy = *start;
        while xy.1 <= max_y + floor {
            let nxy = [(0, 1), (-1, 1), (1, 1)]
                .iter()
                .map(|dxy| (xy.0 + dxy.0, xy.1 + dxy.1))
                .fold(xy, |pos, nxy| {
                    if xy != pos
                        || plane.get(&nxy).is_some()
                        || (floor > 0 && nxy.1 == max_y + floor)
                    {
                        pos
                    } else {
                        nxy
                    }
                });
            if xy == nxy {
                rest = true;
                plane.insert(xy, '°');
                break;
            }
            xy = nxy;
        }
        if floor == 0 {
            if !rest {
                break;
            }
        } else if xy == *start {
            break;
        }
    }
}

fn step1() {
    let input = read_input();
    let mut plane = parse_plane(&input);
    let init = plane.len();
    fill_plane(&mut plane, &(500, 0), 0);
    let fini = plane.len();
    let res = fini - init;
    println!("step1: {res}");
}

fn step2() {
    let input = read_input();
    let mut plane = parse_plane(&input);
    let init = plane.len();
    fill_plane(&mut plane, &(500, 0), 2);
    let fini = plane.len();
    let res = fini - init;
    println!("step2: {res}");
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
    println!("x in ({}, {})", min_x, max_x);
    println!("y in ({}, {})", min_y, max_y);
    for y in min_y..=max_y {
        let line = (min_x..=max_x)
            .map(|x| {
                if let Some(c) = plane.get(&(x, y)) {
                    c.to_string()
                } else {
                    " ".to_string()
                }
            })
            .join("");
        println!("{}", line);
    }
}
