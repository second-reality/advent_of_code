use itertools::EitherOrBoth::*;
use itertools::Itertools;
use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::Chars;

//const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
enum NList {
    Int(u32),
    List(Vec<NList>),
}

type NListPair = (NList, NList);

fn parse_int(chars: &mut Peekable<Chars>) -> Option<NList> {
    let mut val = 0;
    while let Some(d) = chars.peek()?.to_digit(10) {
        val = val * 10 + d;
        chars.next();
    }
    Some(NList::Int(val))
}

fn parse_seq(chars: &mut Peekable<Chars>) -> Option<NList> {
    let mut seq: Vec<NList> = vec![];
    assert!(chars.next()? == '[');
    loop {
        let ch = *chars.peek()?;
        if ch.is_ascii_digit() {
            seq.push(parse_int(chars)?);
        } else if ch == '[' {
            seq.push(parse_seq(chars)?);
        } else {
            chars.next();
            if ch == ']' {
                break;
            }
        }
    }
    Some(NList::List(seq))
}

fn parse_list(line: &str) -> NList {
    let mut chars = line.chars().peekable();
    parse_seq(&mut chars).unwrap()
}

fn parse_pair(line_pair: &str) -> (NList, NList) {
    line_pair
        .split('\n')
        .map(parse_list)
        .collect_tuple()
        .unwrap()
}

fn read_list_pairs() -> Vec<NListPair> {
    INPUT
        .trim()
        .split("\n\n")
        .map(str::to_string)
        .map(|x| parse_pair(x.as_str()))
        .collect_vec()
}

fn compare_elt(a: &NList, b: &NList) -> Ordering {
    match a {
        NList::Int(ia) => match b {
            NList::Int(ib) => ia.cmp(ib),
            NList::List(vb) => compare_velt(&[NList::Int(*ia)], vb),
        },
        NList::List(va) => match b {
            NList::Int(ib) => compare_velt(va, &[NList::Int(*ib)]),
            NList::List(vb) => compare_velt(va, vb),
        },
    }
}

fn compare_velt(a: &[NList], b: &[NList]) -> Ordering {
    a.iter()
        .zip_longest(b.iter())
        .fold(Ordering::Equal, |cmp, pair| match cmp {
            Ordering::Equal => match pair {
                Both(l, r) => compare_elt(l, r),
                Left(_) => Ordering::Greater,
                Right(_) => Ordering::Less,
            },
            _ => cmp,
        })
}

fn compute_sum(list_pairs: &[NListPair]) -> usize {
    list_pairs
        .iter()
        .enumerate()
        .filter_map(|(x, (l, r))| {
            if compare_elt(l, r).is_le() {
                Some(x + 1)
            } else {
                None
            }
        })
        .sum()
}

fn step1() {
    let list_pairs = read_list_pairs();
    let sum = compute_sum(&list_pairs);
    println!("step1: {sum}");
}

fn read_mark_lists() -> Vec<(Option<()>, NList)> {
    let mut list = INPUT
        .trim()
        .split('\n')
        .filter_map(|x| {
            if !x.is_empty() {
                Some((None, parse_list(x)))
            } else {
                None
            }
        })
        .collect_vec();
    list.push((Some(()), parse_list("[[2]]")));
    list.push((Some(()), parse_list("[[6]]")));
    list
}

fn prod_lists(list: &[(Option<()>, NList)]) -> usize {
    list.iter()
        .sorted_by(|(_, a), (_, b)| compare_elt(a, b))
        .enumerate()
        .map(|(i, (m, _))| match m {
            Some(_) => i + 1,
            _ => 1,
        })
        .product()
}

fn step2() {
    let lists = read_mark_lists();
    let prod = prod_lists(&lists);
    println!("step2: {prod}");
}

fn main() {
    step1();
    step2();
}
