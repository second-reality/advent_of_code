#[derive(Debug)]
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

fn main() {
    assert_eq!(
        739785,
        Game::new(Player::new(4), Player::new(8), Dice::new()).part1()
    );
    println!(
        "{}",
        Game::new(Player::new(2), Player::new(1), Dice::new()).part1()
    );
}
