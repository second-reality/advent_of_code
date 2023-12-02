use itertools::Itertools;
use regex::Regex;

//const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");
const STEP1: u32 = 2720;
const STEP2: u32 = 71535;

type Rgb = (u32, u32, u32);

fn read_input() -> Vec<String> {
    INPUT.trim().split('\n').map(str::to_string).collect()
}

fn parse_games(lines: &[String]) -> Vec<(u32, Vec<Rgb>)> {
    let re_game = Regex::new(r"Game (\d+)").unwrap();
    let re_red = Regex::new(r"(\d+) red").unwrap();
    let re_green = Regex::new(r"(\d+) green").unwrap();
    let re_blue = Regex::new(r"(\d+) blue").unwrap();
    lines
        .iter()
        .map(|line| {
            let game_trials = line.split(':').collect::<Vec<_>>();
            let game_id = re_game.captures(game_trials[0]).unwrap()[1]
                .parse::<u32>()
                .unwrap();
            let trials = game_trials[1].split(';').collect::<Vec<_>>();
            let res = vec![&re_red, &re_green, &re_blue];
            let cubes = trials
                .iter()
                .map(|t| {
                    res.iter()
                        .map(|re| match re.captures(t) {
                            Some(x) => x[1].parse::<u32>().unwrap(),
                            _ => 0,
                        })
                        .collect_tuple()
                        .unwrap()
                })
                .collect::<Vec<_>>();
            (game_id, cubes)
        })
        .collect()
}

fn valid_games(games: &[(u32, Vec<Rgb>)], rm: u32, gm: u32, bm: u32) -> Vec<u32> {
    games
        .iter()
        .filter_map(|(id, trials)| {
            if !trials.iter().any(|(r, g, b)| *r > rm || *g > gm || *b > bm) {
                Some(*id)
            } else {
                None
            }
        })
        .collect()
}

fn step1() {
    let input = read_input();
    let games = parse_games(&input);
    let valids = valid_games(&games, 12, 13, 14);
    let res: u32 = valids.iter().sum();
    println!("step1: {res}");
    assert!(res == STEP1);
}

fn minimums(games: &[(u32, Vec<Rgb>)]) -> Vec<Rgb> {
    games
        .iter()
        .map(|g| {
            let (_, trials) = g;
            let (mut rm, mut gm, mut bm) = (0, 0, 0);
            for &(r, g, b) in trials.iter() {
                if r > rm {
                    rm = r
                }
                if g > gm {
                    gm = g
                }
                if b > bm {
                    bm = b
                }
            }
            (rm, gm, bm)
        })
        .collect()
}

fn powers(mins: &[Rgb]) -> Vec<u32> {
    mins.iter().map(|(r, g, b)| r * g * b).collect()
}

fn step2() {
    let input = read_input();
    let games = parse_games(&input);
    let mins = minimums(&games);
    let pows = powers(&mins);
    let res: u32 = pows.iter().sum();
    println!("step2: {res}");
    assert!(res == STEP2);
}

fn main() {
    step1();
    step2();
}
