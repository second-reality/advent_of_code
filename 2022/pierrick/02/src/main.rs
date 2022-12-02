use std::collections::HashMap;

#[derive(Copy, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

type Input = Vec<(Choice, Choice)>;

fn read_choice(s: &str, mapping: &HashMap<&str, Choice>) -> Choice {
    *mapping.get(s).unwrap()
}

fn input<F>(our_choice: F) -> Input
where
    F: Fn(Choice, &str) -> Choice,
{
    let map_theirs = HashMap::from([
        ("A", Choice::Rock),
        ("B", Choice::Paper),
        ("C", Choice::Scissors),
    ]);

    let input_str = include_str!("../input");
    input_str
        .lines()
        .map(|s| {
            let game: Vec<&str> = s.split_whitespace().collect();
            let theirs = read_choice(game[0], &map_theirs);
            let ours = our_choice(theirs, game[1]);
            (theirs, ours)
        })
        .collect()
}

fn game(i: &Input) -> usize {
    let mut score = 0;
    const WIN: usize = 6;
    const DRAW: usize = 3;
    const LOSE: usize = 0;
    for play in i {
        let points = match play {
            (Choice::Rock, Choice::Rock) => 1 + DRAW,
            (Choice::Rock, Choice::Paper) => 2 + WIN,
            (Choice::Rock, Choice::Scissors) => 3 + LOSE,
            (Choice::Paper, Choice::Rock) => 1 + LOSE,
            (Choice::Paper, Choice::Paper) => 2 + DRAW,
            (Choice::Paper, Choice::Scissors) => 3 + WIN,
            (Choice::Scissors, Choice::Rock) => 1 + WIN,
            (Choice::Scissors, Choice::Paper) => 2 + LOSE,
            (Choice::Scissors, Choice::Scissors) => 3 + DRAW,
        };
        score += points;
    }
    score
}

fn part1() -> usize {
    let map_ours = HashMap::from([
        ("X", Choice::Rock),
        ("Y", Choice::Paper),
        ("Z", Choice::Scissors),
    ]);
    game(&input(|_, ours| read_choice(ours, &map_ours)))
}

fn part2() -> usize {
    game(&input(|theirs, ours| match (theirs, ours) {
        (_, "Y") => theirs,
        (Choice::Rock, "X") => Choice::Scissors,
        (Choice::Rock, "Z") => Choice::Paper,
        (Choice::Paper, "X") => Choice::Rock,
        (Choice::Paper, "Z") => Choice::Scissors,
        (Choice::Scissors, "X") => Choice::Paper,
        (Choice::Scissors, "Z") => Choice::Rock,
        (_, _) => unreachable!(),
    }))
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
