use std::{collections::HashSet, str::FromStr};
#[derive(Clone)]
struct JetPattern {
    offsets: Vec<i32>,
    i: usize,
}
impl FromStr for JetPattern {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(JetPattern {
            offsets: s
                .chars()
                .map(|c| match c {
                    '>' => 1,
                    '<' => -1,
                    _ => panic!("Invalid char"),
                })
                .collect(),
            i: 0,
        })
    }
}

impl JetPattern {
    fn next(&mut self) -> i32 {
        let res = self.offsets[self.i];
        self.i = (self.i + 1) % self.offsets.len();
        res
    }
}

type Coord = (i32, i32);

#[derive(Clone)]
struct Tetris {
    rocks: [Vec<Coord>; 5],
    i: usize,
    jet_pattern: JetPattern,
    board: HashSet<Coord>,
    y_highest: i32,
    y_floor: i32,
}

impl Tetris {
    fn new(jet_pattern: JetPattern) -> Self {
        Tetris {
            rocks: [
                vec![(0, 0), (1, 0), (2, 0), (3, 0)],
                vec![(1, 2), (0, 1), (1, 1), (2, 1), (1, 0)],
                vec![(2, 2), (2, 1), (2, 0), (1, 0), (0, 0)],
                vec![(0, 3), (0, 2), (0, 1), (0, 0)],
                vec![(0, 1), (1, 1), (0, 0), (1, 0)],
            ],
            i: 0,
            jet_pattern,
            board: (0..=8)
                .map(|x| (x, 0))
                .chain((0..5).flat_map(|y| [(0, y), (8, y)]))
                .collect(),
            y_highest: 0,
            y_floor: 0,
        }
    }
    fn next_rock(&mut self) {
        // translate rocks templates from relative to abolute coordinates
        let mut rock: Vec<Coord> = self.rocks[self.i]
            .iter()
            .map(|(relx, rely)| (relx + 3, rely + self.y_highest + 4))
            .collect();
        // add walls
        self.board
            .extend((1..=5).flat_map(|dy| [(0, self.y_highest + dy), (8, self.y_highest + dy)]));
        self.i = (self.i + 1) % 5;

        // pushed by jet then fall logic
        while !rock.iter().any(|coord| self.board.contains(coord)) {
            let windx = self.jet_pattern.next();
            rock.iter_mut().for_each(|coord| coord.0 += windx);
            if rock.iter().any(|coord| self.board.contains(coord)) {
                rock.iter_mut().for_each(|coord| coord.0 -= windx);
            }
            rock.iter_mut().for_each(|coord| {
                coord.1 -= 1;
            });
        }
        // rollback last invalid fall and update highest
        rock.iter_mut().for_each(|coord| {
            coord.1 += 1;
            self.y_highest = self.y_highest.max(coord.1);
        });
        let y_min = rock.last().unwrap().1;
        // rocks is now stuck so add it to the walls/static blocks
        self.board.extend(rock);

        // remove points below the new floor if blocks complete a row
        if let Some(y_new_floor) =
            (y_min..=(y_min + 2)).find(|y| (1..=7).all(|x| self.board.contains(&(x, *y))))
        {
            self.board.retain(|(_, y)| y >= &y_new_floor);
            self.y_floor = y_new_floor;
        }
    }

    //
    // fn print(&self) {
    //     for y in (0..=self.y_highest).rev() {
    //         for x in 0..=8 {
    //             let c = if self.board.contains(&(x, y)) {
    //                 '#'
    //             } else {
    //                 '.'
    //             };
    //             print!("{c}");
    //         }
    //         println!("");
    //     }
    // }
}
pub fn part1(input: String) -> usize {
    let mut tetris = Tetris::new(input.trim().parse().unwrap());
    for _in in 0..2022 {
        tetris.next_rock();
    }
    tetris.y_highest as usize
}

fn detect_pattern(record: &Vec<usize>) -> (&[usize], &[usize]) {
    for pat_start in 0..(record.len()) {
        let sub_record = &record[pat_start..];
        for pat_len in 2..(sub_record.len() / 2) {
            if (pat_len..(sub_record.len() - pat_len))
                .step_by(pat_len)
                .all(|p| sub_record[..pat_len] == sub_record[p..(p + pat_len)])
            {
                return (
                    &record[..pat_start],
                    &record[pat_start..(pat_start + pat_len)],
                );
            }
        }
    }
    panic!("Pattern not found");
}

pub fn part2(input: String) -> usize {
    let mut tetris = Tetris::new(input.trim().parse().unwrap());
    let record: Vec<usize> = (0..5_000)
        .map(|_| {
            let high_before = tetris.y_highest;
            tetris.next_rock();
            (tetris.y_highest - high_before) as usize
        })
        .collect();
    let (before_cycle, cycle) = detect_pattern(&record);
    let n_rocks = 1_000_000_000_000;
    let modulo = (n_rocks - before_cycle.len()) % cycle.len();
    let n_cycles = (n_rocks - before_cycle.len()) / cycle.len();
    let high_before_cycle: usize = before_cycle.iter().sum();
    let high_cycle: usize = cycle.iter().sum();
    let rest: usize = cycle[0..modulo].iter().sum();
    high_before_cycle + n_cycles * high_cycle + rest
}

pub const EXPECTED1: usize = 3068;
pub const EXPECTED2: usize = 1514285714288;
