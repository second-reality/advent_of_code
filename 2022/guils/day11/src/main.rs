use itertools::Itertools;

//const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");

type Id = usize;
type Level = i64;
type Op = (char, Level);
type Action = (Op, Id, Id);
type Monkey = (Vec<Level>, Op, Action);

fn read_input() -> Vec<String> {
    INPUT.trim().split("\n\n").map(str::to_string).collect()
}

fn parse_monkeys(input: &[String]) -> Vec<Monkey> {
    input
        .iter()
        .map(|x| {
            let ops = x.split('\n').map(str::trim).collect_vec();
            let items = ops[1]
                .replace("Starting items: ", "")
                .split(", ")
                .flat_map(str::parse::<Level>)
                .collect_vec();
            let (opr, tok) = ops[2]
                .replace("Operation: new = old ", "")
                .split(' ')
                .map(str::to_string)
                .collect_tuple()
                .unwrap();
            let op = if tok == "old" {
                ('S', 0)
            } else {
                (opr.chars().collect_vec()[0], tok.parse::<Level>().unwrap())
            };
            let test = (
                '/',
                ops[3]
                    .replace("Test: divisible by ", "")
                    .parse::<Level>()
                    .unwrap(),
            );
            let if_true = ops[4]
                .replace("If true: throw to monkey ", "")
                .parse::<Id>()
                .unwrap();
            let if_false = ops[5]
                .replace("If false: throw to monkey ", "")
                .parse::<Id>()
                .unwrap();
            let action = (test, if_true, if_false);
            (items, op, action)
        })
        .collect_vec()
}

fn update_val_dst(
    level: Level,
    (opr, val): Op,
    (topr, tval): Op,
    adjust: Level,
    lcm: Level,
) -> (Level, bool) {
    let new = match opr {
        '+' => level + val,
        '*' => level * val,
        'S' => level * level,
        _ => unreachable!(),
    } / adjust
        % lcm;
    let test = match topr {
        '/' => new % tval == 0,
        _ => unreachable!(),
    };
    (new, test)
}

fn execute_round(mks: &[Monkey], adjust: Level) -> (Vec<Monkey>, Vec<usize>) {
    let mut new_mks = mks.to_vec();
    let mut counts = vec![0; new_mks.len()];
    let lcm = mks.iter().map(|(_, _, ((_, div), _, _))| div).product();
    mks.iter()
        .enumerate()
        .for_each(|(id, (_, op, (top, ift, iff)))| {
            new_mks[id].0.clone().iter().for_each(|level| {
                let (new, test) = update_val_dst(*level, *op, *top, adjust, lcm);
                let dst = if test { *ift } else { *iff };
                new_mks[id].0.remove(0);
                new_mks[dst].0.push(new);
                counts[id] += 1;
            })
        });
    (new_mks, counts)
}

fn execute_rounds(mks: &[Monkey], adjust: Level, n: usize) -> (Vec<Monkey>, Vec<usize>) {
    (0..n).fold((mks.to_vec(), vec![0; mks.len()]), |(mks, count), _| {
        let (new_mks, new_count) = execute_round(&mks, adjust);
        let all_count = count
            .iter()
            .zip(new_count.iter())
            .map(|(x, y)| x + y)
            .collect();
        (new_mks, all_count)
    })
}

fn get_level(counts: &[usize]) -> usize {
    counts.iter().sorted().rev().take(2).product()
}

fn step1() {
    let input = read_input();
    let mks = parse_monkeys(&input);
    let (_mks_20, counts) = execute_rounds(&mks, 3, 20);
    let res = get_level(&counts);
    println!("step1: {res}");
}

fn step2() {
    let input = read_input();
    let mks = parse_monkeys(&input);
    let (_mks_20, counts) = execute_rounds(&mks, 1, 10000);
    let res = get_level(&counts);
    println!("step2: {res}");
}

fn main() {
    step1();
    step2();
}
