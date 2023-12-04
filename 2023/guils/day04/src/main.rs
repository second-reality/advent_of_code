use itertools::Itertools;
use std::collections::HashSet;

// const INPUT: &str = include_str!("../test.txt");
// const STEP1: u32 = 13;
// const STEP2: u32 = 30;
const INPUT: &str = include_str!("../input.txt");
const STEP1: u32 = 25183;
const STEP2: u32 = 5667240;

fn read_input() -> Vec<String> {
    INPUT.trim().split('\n').map(str::to_string).collect()
}

fn cards_wins(lines: &[String]) -> Vec<usize> {
    lines
        .iter()
        .map(|l| {
            let (_card, game) = l.split(':').map(str::trim).collect_tuple().unwrap();
            let (wins, cards) = game.split('|').map(str::trim).collect_tuple().unwrap();
            let wins_set = wins
                .split(' ')
                .filter_map(|x| x.parse::<u32>().ok())
                .collect::<HashSet<_>>();
            cards
                .split(' ')
                .filter_map(|x| x.parse::<u32>().ok())
                .filter(|c| wins_set.contains(c))
                .count()
        })
        .collect()
}

fn cards_sum1(cards: &[usize]) -> u32 {
    cards
        .iter()
        .map(|x| if *x == 0 { 0 } else { 2u32.pow(*x as u32 - 1) })
        .sum()
}

fn step1() {
    let input = read_input();
    let draws = cards_wins(&input);
    let res = cards_sum1(&draws);
    println!("step1: {res}");
    assert!(res == STEP1);
}

fn cards_play(cards: &[usize]) -> u32 {
    let mut scratch = cards.iter().enumerate().collect::<Vec<_>>();
    let mut sum = cards.len();
    while let Some((id, wins)) = scratch.pop() {
        sum += wins;
        for (nid, nwins) in cards
            .iter()
            .enumerate()
            .skip(id + 1)
            .take(*wins)
            .filter(|(_, wins)| **wins > 0)
        {
            scratch.push((nid, nwins));
        }
    }
    sum as u32
}

fn step2() {
    let input = read_input();
    let draws = cards_wins(&input);
    let res = cards_play(&draws);
    println!("step2: {res}");
    assert!(res == STEP2);
}

fn main() {
    step1();
    step2();
}
