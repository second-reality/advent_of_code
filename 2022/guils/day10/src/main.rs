use itertools::Itertools;
use std::iter;

//const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");

type Val = i32;
type Op = (String, Val, usize);
type Pixel = char;

fn read_input() -> Vec<String> {
    INPUT.trim().split('\n').map(str::to_string).collect()
}

fn parse_ops(input: &[String]) -> Vec<Op> {
    input
        .iter()
        .map(|line| {
            let op = line.split(' ').collect_vec();
            match op[0] {
                "noop" => (op[0].to_string(), 0, 1),
                "addx" => (op[0].to_string(), op[1].parse::<i32>().unwrap(), 2),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn compute_x(ops: &[Op]) -> Vec<Val> {
    let mut x = 1;
    ops.iter()
        .flat_map(|(op, v, n)| {
            let (xo, xn) = (
                x,
                match op.as_str() {
                    "addx" => x + v,
                    _ => x,
                },
            );
            x = xn;
            iter::repeat(xo).take(*n)
        })
        .collect()
}

fn step1() {
    let input = read_input();
    let ops = parse_ops(&input);
    let xs = compute_x(&ops);
    let res = vec![20, 60, 100, 140, 180, 220]
        .iter()
        .map(|c| *c as i32 * xs[c - 1])
        .sum::<i32>();
    println!("step1: {res}");
}

fn compute_crt(xs: &[Val]) -> Vec<Pixel> {
    xs.iter()
        .enumerate()
        .map(|(c, x)| {
            let cc = (c % 40 + 1) as i32;
            if cc >= *x && cc <= *x + 2 {
                '#'
            } else {
                ' '
            }
        })
        .collect()
}

fn step2() {
    let input = read_input();
    let ops = parse_ops(&input);
    let xs = compute_x(&ops);
    let pix = compute_crt(&xs);
    println!("step2: Read Chars Below ---v");
    print_crt(&pix);
}

fn main() {
    step1();
    step2();
}

fn print_crt(pix: &[Pixel]) {
    pix.to_vec().chunks(40).fold((), |_, it| {
        println!("{}", it.iter().join(""));
    })
}
