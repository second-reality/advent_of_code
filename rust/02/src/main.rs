use std::str::FromStr;

enum Direction {
    Up,
    Down,
    Forward,
}

struct Movement {
    direction: Direction,
    increment: i32,
}

struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    fn apply_move_part1(&mut self, m: &Movement) {
        match m.direction {
            Direction::Up => self.depth -= m.increment,
            Direction::Down => self.depth += m.increment,
            Direction::Forward => self.horizontal += m.increment,
        }
    }

    fn apply_move_part2(&mut self, m: &Movement) {
        match m.direction {
            Direction::Up => self.aim -= m.increment,
            Direction::Down => self.aim += m.increment,
            Direction::Forward => {
                self.horizontal += m.increment;
                self.depth += self.aim * m.increment;
            }
        }
    }
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            "forward" => Ok(Direction::Forward),
            _ => Err(()),
        }
    }
}

fn parse_line(line: &str) -> Movement {
    let s: Vec<&str> = line.split_whitespace().collect();
    assert!(s.len() == 2);
    let dir = FromStr::from_str(s[0]).unwrap();
    let inc = s[1].parse().unwrap();
    Movement {
        direction: dir,
        increment: inc,
    }
}

fn parse_input() -> Vec<Movement> {
    let input_str = include_str!("../input.txt");
    input_str.lines().map(parse_line).collect()
}

fn part1(moves: &[Movement]) -> i32 {
    let mut p: Position = Position {
        horizontal: 0,
        depth: 0,
        aim: 0, // not important here
    };
    for m in moves {
        p.apply_move_part1(m);
    }
    p.depth * p.horizontal
}

fn part2(moves: &[Movement]) -> i32 {
    let mut p: Position = Position {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };
    for m in moves {
        p.apply_move_part2(m);
    }
    p.depth * p.horizontal
}

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
