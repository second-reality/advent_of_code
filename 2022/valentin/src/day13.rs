use std::{cmp::Ordering, ops::Range};
#[derive(Debug, Clone, PartialEq, Eq)]
enum Expr {
    Exprs(Vec<Expr>),
    Int(u8),
}

impl Expr {
    fn new(line: &str) -> Self {
        if let Ok(int) = line.parse() {
            Expr::Int(int)
        } else if line.len() == 2 {
            Expr::Exprs(Vec::new())
        } else {
            let line = &line[1..(line.len() - 1)];
            let mut depth = 0;
            let mut partition: Vec<Range<usize>> = Vec::new();
            let mut prev = 0;
            for (i, ch) in line.chars().enumerate() {
                if ch == '[' {
                    depth += 1;
                } else if ch == ']' {
                    depth -= 1;
                } else if ch == ',' && depth == 0 {
                    partition.push(prev..i);
                    prev = i + 1;
                }
            }
            partition.push(prev..(line.len()));
            Expr::Exprs(
                partition
                    .into_iter()
                    .map(|range| Expr::new(&line[range]))
                    .collect(),
            )
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        use Expr::*;
        match (self, other) {
            (Int(a), Int(b)) => {
                println!("cmp Left = {}, Right = {}", a, b);
                a.cmp(&b)
            }
            (Exprs(left), Exprs(right)) => {
                let n = left.len().min(right.len());
                println!("Exprs to Exprs n = {}", n);
                for i in 0..n {
                    match left[i].cmp(&right[i]) {
                        Ordering::Equal => continue,
                        o => {
                            return o;
                        }
                    }
                }
                left.len().cmp(&right.len())
            }
            (Int(left), right) => {
                println!("Exprs to int");
                Exprs(vec![Int(*left)]).cmp(right)
            }
            (left, Int(right)) => {
                println!("Exprs to int");
                left.cmp(&Exprs(vec![Int(*right)]))
            }
        }
    }
}

pub fn part1(input: String) -> usize {
    input
        .trim()
        .split("\n\n")
        .enumerate()
        .map(|(i, pair)| {
            println!("{}", pair);
            let mut pair = pair.split('\n');
            let left = Expr::new(pair.next().unwrap());
            let right = Expr::new(pair.next().unwrap());
            let t = match left.cmp(&right) {
                Ordering::Less => i + 1,
                Ordering::Greater => 0,
                Ordering::Equal => panic!("lul"),
            };
            println!("t = {}", t);
            t
        })
        .sum()
}

pub fn part2(input: String) -> usize {
    let mut packets: Vec<Expr> = input
        .trim()
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| Expr::new(line))
        .collect();
    let divider2 = Expr::new("[[2]]");
    let divider6 = Expr::new("[[6]]");
    packets.push(divider2.clone());
    packets.push(divider6.clone());
    packets.sort_by(|a, b| a.cmp(b));
    packets
        .into_iter()
        .enumerate()
        .filter(|(_, p)| *p == divider2 || *p == divider6)
        .map(|(i, _)| i + 1)
        .product()
}

pub const EXPECTED1: usize = 13;
pub const EXPECTED2: usize = 140;
