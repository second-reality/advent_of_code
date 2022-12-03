fn read_input() -> Vec<(i32, i32)> {
    //include_str!("../test.txt")
    include_str!("../input.txt")
        .trim()
        .split('\n')
        .map(|x| {
            let its = x
                .split(' ')
                .map(|x| map_play(x.to_string()))
                .collect::<Vec<_>>();
            (its[0], its[1])
        })
        .collect()
}

const R: i32 = 1;
const P: i32 = 2;
const S: i32 = 3;
const L: i32 = 1;
const D: i32 = 2;
const W: i32 = 3;
const LS: i32 = 0;
const DS: i32 = 3;
const WS: i32 = 6;

fn map_play(play: String) -> i32 {
    match play.as_str() {
        "A" | "X" => R,
        "B" | "Y" => P,
        "C" | "Z" => S,
        _ => panic!(),
    }
}

fn rnd_out(rnd: (i32, i32)) -> i32 {
    if rnd.0 == rnd.1 {
        return DS;
    }
    match rnd {
        (R, P) => WS,
        (P, S) => WS,
        (S, R) => WS,
        _ => rnd_out((rnd.1, rnd.0)) - WS + LS,
    }
}

fn rnd_score(rnd: (i32, i32)) -> i32 {
    rnd.1 + rnd_out(rnd)
}

fn step1() {
    let rnds = read_input();
    let score = rnds.iter().map(|&x| rnd_score(x)).sum::<i32>();
    println!("step1: {score}");
}

fn out_map_rnd(rnd: (i32, i32)) -> (i32, i32) {
    match rnd {
        (R, W) => (R, P),
        (R, L) => (R, S),
        (P, L) => (P, R),
        (P, W) => (P, S),
        (S, L) => (S, P),
        (S, W) => (S, R),
        (o, D) => (o, o),
        _ => panic!(),
    }
}

fn step2() {
    let rnds = read_input();
    let score = rnds.iter().map(|&x| rnd_score(out_map_rnd(x))).sum::<i32>();
    println!("step2: {score}");
}

fn main() {
    step1();
    step2();
}
