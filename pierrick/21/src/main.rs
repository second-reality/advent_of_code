use itertools::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Player {
    position: usize,
    score: usize,
}

impl Player {
    fn new(init_pos: usize) -> Self {
        Player {
            position: init_pos,
            score: 0,
        }
    }

    fn play(&mut self, num_pos: usize) {
        for _ in 0..num_pos {
            if self.position == 10 {
                self.position = 0;
            }
            self.position += 1;
        }
        self.score += self.position;
    }
}

#[derive(Debug)]
struct Dice {
    last: usize, // start at zero
    num_rolled: usize,
}

impl Dice {
    fn new() -> Self {
        Dice {
            last: 0,
            num_rolled: 0,
        }
    }

    fn roll(&mut self) -> usize {
        let next = if self.last == 100 { 1 } else { self.last + 1 };
        self.last = next;
        self.num_rolled += 1;
        next
    }
}

#[derive(Debug)]
struct Game {
    players: [Player; 2],
    dice: Dice,
}

impl Game {
    fn new(p1: Player, p2: Player, dice: Dice) -> Self {
        Game {
            players: [p1, p2],
            dice,
        }
    }

    fn play(&mut self) {
        assert!(!self.finished());
        let mut player_id = 0;
        while !self.finished() {
            self.play_one(player_id);
            player_id += 1;
            player_id %= self.players.len();
        }
    }

    fn play_one(&mut self, player_id: usize) {
        let mut bump = 0;
        for _ in 0..3 {
            bump += self.dice.roll();
        }
        self.players[player_id].play(bump);
        //println!("{:?}", self);
    }

    fn finished(&self) -> bool {
        self.players.iter().any(|p| p.score >= 1000)
    }

    fn part1(&mut self) -> usize {
        self.play();
        self.dice.num_rolled * self.players.iter().map(|p| p.score).min().unwrap()
    }
}

type Cache = HashMap<(Player, Player), (usize, usize)>;

fn play_part2(p1: Player, p2: Player, cache: &mut Cache) -> (usize, usize) {
    if p1.score >= 21 {
        return (1, 0);
    }
    if p2.score >= 21 {
        return (0, 1);
    }

    let mut res = (0, 0);

    // every roll can be 1, 2, 3
    // and it's done 3 times
    for (a, b, c) in iproduct!(1..4, 1..4, 1..4) {
        let outcomes = [a, b, c];
        let bump = outcomes.into_iter().sum();
        let mut next = p1;
        next.play(bump);
        if !cache.contains_key(&(p2, next)) {
            let val = play_part2(p2, next, cache);
            cache.insert((p2, next), val);
        }
        let (win_p2, win_p1) = cache.get(&(p2, next)).unwrap();
        res.0 += win_p1;
        res.1 += win_p2;
    }

    res
}

fn part2(p1: Player, p2: Player) -> usize {
    let mut cache = Cache::new();
    let (p1_win, p2_win) = play_part2(p1, p2, &mut cache);
    usize::max(p1_win, p2_win)
}

fn main() {
    assert_eq!(
        739785,
        Game::new(Player::new(4), Player::new(8), Dice::new()).part1()
    );
    println!(
        "{}",
        Game::new(Player::new(2), Player::new(1), Dice::new()).part1()
    );
    assert_eq!(444356092776315, part2(Player::new(4), Player::new(8)));
    println!("{}", part2(Player::new(2), Player::new(1)));
}
