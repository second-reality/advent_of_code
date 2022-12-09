use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

//const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");

type Coord = (usize, usize);
type CoordMap = HashMap<Coord, i32>;
type Plane = (Coord, CoordMap);
type CoordSet = HashSet<Coord>;

fn read_input() -> Vec<String> {
    INPUT.trim().split('\n').map(str::to_string).collect()
}

fn parse_plane(input: &[String]) -> Plane {
    (
        (input.len(), input[0].len()),
        input
            .iter()
            .enumerate()
            .flat_map(|(l, line)| {
                line.chars()
                    .map(|x| x.to_digit(10).unwrap() as i32)
                    .enumerate()
                    .map(move |(c, x)| ((l, c), x))
            })
            .collect(),
    )
}

fn visible_from_left((dim, points): &Plane) -> CoordSet {
    (0..dim.0).fold(CoordSet::new(), |pset, l| {
        (0..dim.1)
            .fold((pset, -1), |(mut pset, max), c| {
                let h = points.get(&(l, c)).unwrap();
                if *h > max {
                    pset.insert((l, c));
                    (pset, *h)
                } else {
                    (pset, max)
                }
            })
            .0
    })
}

fn rotate_left((dim, points): &Plane) -> Plane {
    (
        (dim.1, dim.0),
        points
            .iter()
            .map(|(&(l, c), &v)| ((dim.1 - 1 - c, l), v))
            .collect(),
    )
}

fn rotate_set_left(dim: &Coord, pset: &CoordSet) -> CoordSet {
    pset.iter().map(|&(l, c)| (dim.1 - 1 - c, l)).collect()
}

fn visibles(plane: &Plane) -> CoordSet {
    (0..4)
        .fold((CoordSet::new(), plane.clone()), |(mut pset, plane), _| {
            let set = visible_from_left(&plane);
            pset.extend(set.iter());
            (rotate_set_left(&plane.0, &pset), rotate_left(&plane))
        })
        .0
}

fn step1() {
    let input = read_input();
    let plane = parse_plane(&input);
    let visibles = visibles(&plane);
    let res = visibles.len();
    println!("step1: {res}");
}

fn count_from_right((dim, points): &Plane) -> CoordMap {
    (0..dim.0).fold(CoordMap::new(), |hset, l| {
        (0..dim.1).fold(hset, |mut hset, c| {
            let (_, count) = (c..dim.1).fold((0, 0), |(mut h_c, mut count), c_idx| {
                let h = points.get(&(l, c_idx)).unwrap();
                if c_idx == c {
                    h_c = *h;
                } else if h_c != -1 {
                    count += 1;
                    if *h >= h_c {
                        h_c = -1;
                    }
                }
                (h_c, count)
            });
            hset.insert((l, c), count);
            hset
        })
    })
}

fn counts(plane: &Plane) -> CoordMap {
    (0..4)
        .fold((CoordMap::new(), plane.clone()), |(mut cmap, plane), _| {
            let map = count_from_right(&plane);
            cmap = map.iter().fold(cmap, |mut cmap, (coord, v)| {
                cmap.entry(*coord).and_modify(|x| *x *= v).or_insert(*v);
                cmap.to_owned()
            });
            (rotate_left(&(plane.0, cmap)).1, rotate_left(&plane))
        })
        .0
}
fn step2() {
    let input = read_input();
    let plane = parse_plane(&input);
    let counts = counts(&plane);
    let res = counts.values().max().unwrap();
    //print_values(&counts);
    println!("step2: {res}");
}

fn main() {
    step1();
    step2();
}

#[allow(dead_code)]
fn print_values(plane: &CoordMap) {
    let max_c = plane.iter().map(|((_, c), _)| c).max().unwrap();
    let max_l = plane.iter().map(|((l, _), _)| l).max().unwrap();
    for l in 0..max_l + 1 {
        let line = (0..max_c + 1)
            .map(|c| plane.get(&(l, c)).unwrap().to_string())
            .join("");
        println!("{line}");
    }
}

#[allow(dead_code)]
fn print_set(pset: &CoordSet) {
    let max_c = pset.iter().map(|(_, c)| c).max().unwrap();
    let max_l = pset.iter().map(|(l, _)| l).max().unwrap();
    for l in 0..max_l + 1 {
        let line = (0..max_c + 1)
            .map(|c| if pset.contains(&(l, c)) { "X" } else { " " })
            .join("");
        println!("{line}");
    }
}
