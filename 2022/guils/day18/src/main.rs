use itertools::Itertools;
use std::collections::HashSet;

//const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");

type Coord = (i32, i32, i32);
fn read_input() -> Vec<Coord> {
    INPUT.trim().split('\n')
	.map(|x|
	     x.split(',').map(|x| x.parse::<i32>().unwrap()).collect_tuple().unwrap())
	.collect_vec()
}

//fn adjacent(a: &Coord, b:&Coord) -> bool {
//    (a.0 - b.0).abs() +	(a.1 - b.1).abs() + (a.2 - b.2).abs() == 1
//}

fn n_adjacent_set(set: &HashSet<Coord>, x:&Coord) -> i32 {
    let mut n = 0;
    for adj in [(x.0 - 1, x.1, x.2),
		(x.0 + 1, x.1, x.2),
		(x.0, x.1 - 1, x.2),
		(x.0, x.1 + 1, x.2),
		(x.0, x.1, x.2 - 1),
		(x.0, x.1, x.2 + 1)] {
	if set.contains(&adj) {
	    n += 1;
	}
    }
    n
}

fn surface(input: &[Coord]) -> i32
{
    let mut set = HashSet::<Coord>::new();
    let mut surface = 0;
    for x in input.iter() {
	surface += 6;
	surface -= n_adjacent_set(&set, x) * 2;
	set.insert(x.clone());
    }
    surface
}

fn step1() {
    let input = read_input();
    let res = surface(&input);
    println!("step1: {res}");
}

fn step2() {
    let input = read_input();
    let res = input.len();
    println!("step2: {res}");
}

fn main() {
    step1();
    step2();
}
