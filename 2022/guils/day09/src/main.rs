use itertools::Itertools;
use std::cmp;
use std::collections::HashSet;
use std::iter;

//const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");

type Coord = (i32, i32);
type Move = Coord;

fn read_input() -> Vec<String> {
    INPUT.trim().split('\n').map(str::to_string).collect()
}

fn parse_input(input: &[String]) -> Vec<Move> {
    input
        .iter()
        .flat_map(|x| {
            let (dir, n) = x.split(' ').collect_tuple().unwrap();
            iter::repeat(dir).take(n.parse::<usize>().unwrap())
        })
        .map(|dir| match dir {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => unreachable!(),
        })
        .collect()
}

fn move_head(moves: &[Move]) -> Vec<Coord> {
    moves.iter().fold(vec![(0, 0)], |mut coords, m| {
        let (x, y) = coords.last().unwrap();
        coords.push((x + m.0, y + m.1));
        coords
    })
}

fn move_tail(heads: &[Coord]) -> Vec<Coord> {
    heads
        .iter()
        .skip(1)
        .fold(vec![(0, 0)], |mut tails, (hx, hy)| {
            let (x, y) = tails.last().unwrap();
            let (dx, dy) = (hx - x, hy - y);
            let ddx = cmp::max(-1, cmp::min(dx, 1));
            let ddy = cmp::max(-1, cmp::min(dy, 1));
            tails.push(if dx.abs() > 1 || dy.abs() > 1 {
                (ddx + x, ddy + y)
            } else {
                (*x, *y)
            });
            tails
        })
}

fn step1() {
    let input = read_input();
    let moves = parse_input(&input);
    let heads = move_head(&moves);
    let tails = move_tail(&heads);
    let res = tails.iter().collect::<HashSet<_>>().len();
    println!("step1: {res}");
}

fn move_n_tail(heads: &[Coord], knots: i32) -> Vec<Coord> {
    (0..knots).fold(heads.to_vec(), |heads, _| move_tail(&heads))
}

fn step2() {
    let input = read_input();
    let moves = parse_input(&input);
    let heads = move_head(&moves);
    let tails = move_n_tail(&heads, 9);
    let res = tails.iter().collect::<HashSet<_>>().len();
    println!("step1: {res}");
}

fn main() {
    step1();
    step2();
}
