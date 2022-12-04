use std::cmp::{max, min};

struct Assignment {
    lower: usize,
    upper: usize,
}

impl Assignment {
    fn new(s: &str) -> Self {
        let mut bounds = s.split('-');
        let lower = bounds.next().unwrap().parse::<usize>().unwrap();
        let upper = bounds.next().unwrap().parse::<usize>().unwrap();
        Assignment { lower, upper }
    }

    fn contains(&self, other: &Assignment) -> bool {
        other.lower >= self.lower && other.upper <= self.upper
    }

    fn num_overlapping_sections(&self, other: &Assignment) -> usize {
        let min_common = max(self.lower, other.lower);
        let max_common = min(self.upper, other.upper);
        if min_common > max_common {
            0
        } else {
            max_common - min_common + 1
        }
    }
}

type Input = Vec<(Assignment, Assignment)>;

fn input(input_str: &str) -> Input {
    input_str
        .lines()
        .map(|line| {
            let mut assignments = line.split(',');
            (
                Assignment::new(assignments.next().unwrap()),
                Assignment::new(assignments.next().unwrap()),
            )
        })
        .collect()
}

fn part1(i: &Input) -> usize {
    i.iter()
        .filter(|(a, b)| a.contains(b) || b.contains(a))
        .count()
}

fn part2(i: &Input) -> usize {
    i.iter()
        .filter(|(a, b)| a.num_overlapping_sections(b) > 0)
        .count()
}

fn main() {
    let input_str = include_str!("../input");
    let test_str = include_str!("../test");
    let i = input(input_str);
    let t = input(test_str);
    assert_eq!(part1(&t), 2);
    println!("{}", part1(&i));
    assert_eq!(part2(&t), 4);
    println!("{}", part2(&i));
}
