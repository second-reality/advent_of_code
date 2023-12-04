use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

//const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");
const STEP1: u32 = 2720;
const STEP2: u32 = 71535;

lazy_static! {
    static ref RE_GAME: Regex = Regex::new(r"Game (\d+)").unwrap();
    static ref RE_RED: Regex = Regex::new(r"(\d+) red").unwrap();
    static ref RE_GREEN: Regex = Regex::new(r"(\d+) green").unwrap();
    static ref RE_BLUE: Regex = Regex::new(r"(\d+) blue").unwrap();
}

type Rgb = (u32, u32, u32);
type Game = (u32, Vec<Rgb>);

fn read_input() -> Vec<String> {
    INPUT.trim().split('\n').map(str::to_string).collect()
}

fn parse_game(line: &str) -> Game {
    let game_trials = line.split(':').collect::<Vec<_>>();
    let game_id = RE_GAME.captures(game_trials[0]).unwrap()[1]
        .parse::<u32>()
        .unwrap();
    let trials = game_trials[1].split(';').collect::<Vec<_>>();
    let res: Vec<&Regex> = vec![&RE_RED, &RE_GREEN, &RE_BLUE];
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
}

fn parse_games(lines: &[String]) -> Vec<Game> {
    lines.iter().map(|l| parse_game(l)).collect()
}

fn valid_games(games: &[Game], rm: u32, gm: u32, bm: u32) -> Vec<u32> {
    games
        .iter()
        .filter_map(|(id, trials)| {
            (!trials.iter().any(|(r, g, b)| *r > rm || *g > gm || *b > bm)).then_some(*id)
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

fn minimums(games: &[Game]) -> Vec<Rgb> {
    games
        .iter()
        .map(|(_, trials)| {
            trials.iter().fold((0, 0, 0), |(r, g, b), &(x, y, z)| {
                (r.max(x), g.max(y), b.max(z))
            })
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
