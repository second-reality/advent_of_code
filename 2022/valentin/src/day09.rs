use std::{collections::HashSet, str::FromStr};
struct Action {
    sign: i32,
    direction: usize,
    count: i32,
}
impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut t = s.split(' ');
        let direction = t.next().unwrap();
        let count: i32 = t.next().unwrap().parse().unwrap();
        let (sign, direction) = match direction {
            "U" => (1, 1),
            "D" => (-1, 1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => panic!("wtf"),
        };
        Ok(Action {
            sign,
            direction,
            count,
        })
    }
}
fn sign_of(i: i32) -> i32 {
    use std::cmp::Ordering::*;
    match i.cmp(&0) {
        Less => -1,
        Equal => 0,
        Greater => 1,
    }
}
struct Rope {
    size: usize,
    knots: Vec<[i32; 2]>,
    visited: HashSet<[i32; 2]>,
}
impl Rope {
    fn new(size: usize) -> Self {
        Rope {
            size,
            knots: (0..size).map(|_| [0, 0]).collect(),
            visited: HashSet::from([[0, 0]]),
        }
    }
    fn dif(&self, index: usize) -> [i32; 2] {
        let h = self.knots[index];
        let t = self.knots[index + 1];
        [h[0] - t[0], h[1] - t[1]]
    }

    fn apply(&mut self, action: Action) {
        let sign = action.sign;
        let dir = action.direction;
        let count = action.count;
        for _ in 0..count {
            self.knots[0][dir] += sign;
            for index in 0..(self.size - 1) {
                let dif = self.dif(index);
                // not touching
                if dif.iter().any(|c| !(-1..=1).contains(c)) {
                    let mov = dif.map(sign_of);
                    self.knots[index + 1][0] += mov[0];
                    self.knots[index + 1][1] += mov[1];
                }
            }
            self.visited.insert(self.knots[self.size - 1]);
        }
    }
}
pub fn part1(input: String) -> usize {
    let mut rope = Rope::new(2);
    input
        .trim()
        .split('\n')
        .map(|line| line.parse::<Action>().unwrap())
        .for_each(|action| rope.apply(action));
    rope.visited.len()
}

pub fn part2(input: String) -> usize {
    let mut rope = Rope::new(10);
    // let input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20".to_owned();
    input
        .trim()
        .split('\n')
        .map(|line| line.parse::<Action>().unwrap())
        .for_each(|action| rope.apply(action));
    // assert_eq!(36, rope.visited.len());
    rope.visited.len()
}

pub const EXPECTED1: usize = 13;
pub const EXPECTED2: usize = 1;
