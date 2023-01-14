use num_bigint::BigUint;
use num_traits::identities::{One, Zero};

//const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");

type VMap = Vec<(Coord, char)>;
type Coord = (usize, usize);
type BSet = Vec<BigUint>;
type VSets = Vec<BSet>;
type Size = (usize, usize);

struct BSets {
    left: BSet,
    right: BSet,
    down: BSet,
    up: BSet,
}

fn read_input() -> VMap {
    INPUT
        .trim()
        .split('\n')
        .enumerate()
        .flat_map(|(l, line)| {
            line.chars().enumerate().filter_map(
                move |(c, ch)| {
                    if ch != '.' {
                        Some(((l, c), ch))
                    } else {
                        None
                    }
                },
            )
        })
        .collect()
}

fn map_size(map: &VMap) -> Size {
    (
        map.iter().map(|(pos, _)| pos.0).max().unwrap() - 1,
        map.iter().map(|(pos, _)| pos.1).max().unwrap() - 1,
    )
}

fn compute_sets(map: &VMap) -> BSets {
    let (ysize, xsize) = map_size(map);
    let mask = (BigUint::one() << xsize) - BigUint::one();
    let mut left = vec![mask.clone(); ysize];
    let mut right = vec![mask.clone(); ysize];
    let mut down = vec![mask.clone(); ysize];
    let mut up = vec![mask; ysize];
    for ((y, x), ch) in map.iter().filter(|x| x.1 != '#') {
        let set = match ch {
            '>' => &mut right,
            '<' => &mut left,
            'v' => &mut down,
            '^' => &mut up,
            _ => unreachable!(),
        };
        set[y - 1] ^= BigUint::one() << (x - 1);
    }
    BSets {
        left,
        right,
        down,
        up,
    }
}

fn shift_left(bits: &BigUint, xsize: usize, shift: usize) -> BigUint {
    let mask = (BigUint::one() << xsize) - BigUint::one();
    (bits << (shift.min(xsize))) & mask
}

fn shift_right(bits: &BigUint, xsize: usize, shift: usize) -> BigUint {
    bits >> shift.min(xsize)
}

fn rotate_left(bits: &BigUint, xsize: usize, shift: usize) -> BigUint {
    let ls = shift % xsize;
    let rs = xsize - ls;
    shift_left(bits, xsize, ls) | shift_right(bits, xsize, rs)
}

fn rotate_right(bits: &BigUint, xsize: usize, shift: usize) -> BigUint {
    let rs = shift % xsize;
    let ls = xsize - rs;
    shift_left(bits, xsize, ls) | shift_right(bits, xsize, rs)
}

fn valid_pos(sets: &BSets, xsize: usize, ysize: usize, y: usize, step: usize) -> BigUint {
    let (right, left, down, up) = (
        &rotate_left(&sets.right[y], xsize, step),
        &rotate_right(&sets.left[y], xsize, step),
        &sets.down[(y + ysize - (step % ysize)) % ysize],
        &sets.up[(y + step) % ysize],
    );
    right & left & down & up
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn compute_all_sets(sets: &BSets, dim: Size) -> VSets {
    (0..lcm(dim.0, dim.1))
        .map(|t| {
            (0..dim.0)
                .map(|y| valid_pos(sets, dim.1, dim.0, y, t))
                .collect()
        })
        .collect()
}

fn update_pos(
    sets: &VSets,
    dim: Size,
    positions: &BSet,
    start: Coord,
    step: usize,
) -> (BSet, usize) {
    let mut new = positions.clone();
    new[start.0].set_bit(start.1 as u64, true);
    for y in 0..dim.0 {
        let bits = &mut new[y];
        *bits |= &positions[y] << 1;
        *bits |= &positions[y] >> 1;
        if y > 0 {
            *bits |= &positions[y - 1];
        }
        if y < dim.0 - 1 {
            *bits |= &positions[y + 1];
        }
        *bits &= &sets[(step + 1) % sets.len()][y];
    }
    (new, step + 1)
}

fn walk(sets: &VSets, dim: Size, start: Coord, end: Coord, init_step: usize) -> usize {
    let mut positions = vec![BigUint::zero(); dim.0];
    let mut step = init_step;
    while !positions[end.0].bit(end.1 as u64) {
        (positions, step) = update_pos(sets, dim, &positions, start, step);
    }
    step + 1
}

fn step1(sets: &VSets, dim: Size) {
    let (start, end) = ((0, 0), (dim.0 - 1, dim.1 - 1));
    let res = walk(sets, dim, start, end, 0);
    println!("step1: {res}");
}

fn step2(sets: &VSets, dim: Size) {
    let (start, end) = ((0, 0), (dim.0 - 1, dim.1 - 1));
    let step1 = walk(sets, dim, start, end, 0);
    let step2 = walk(sets, dim, end, start, step1);
    let res = walk(sets, dim, start, end, step2);
    println!("step2: {res}");
}

fn main() {
    let map = read_input();
    let dim = map_size(&map);
    let sets = compute_sets(&map);
    let vsets = compute_all_sets(&sets, dim);
    step1(&vsets, dim);
    step2(&vsets, dim);
}
