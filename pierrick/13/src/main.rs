use std::cmp::*;
use std::collections::BTreeSet;

type Coords = (i32, i32);

#[derive(Debug, Clone)]
struct Paper {
    points: BTreeSet<Coords>,
}

#[derive(Debug)]
enum Fold {
    Vertical(i32),
    Horizontal(i32),
}

fn get_input(s: &str) -> (Paper, Vec<Fold>) {
    let points = BTreeSet::from_iter(
        s.lines()
            .filter(|line| line.chars().any(|c| c == ','))
            .map(|line| line.split_once(',').unwrap())
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap())),
    );

    let fold = Vec::from_iter(
        s.lines()
            .filter(|line| line.starts_with("fold along"))
            .map(|line| line.split_once('=').unwrap())
            .map(|(dir, coord)| (dir, coord.parse().unwrap()))
            .map(|(dir, coord)| match dir {
                "fold along x" => Fold::Vertical(coord),
                "fold along y" => Fold::Horizontal(coord),
                _ => unreachable!(),
            }),
    );

    (Paper { points }, fold)
}

impl Paper {
    fn fold(&self, f: &Fold) -> Self {
        let new_points =
            BTreeSet::from_iter(self.points.iter().map(|p| Self::fold_one_point(p, f)));
        Paper { points: new_points }
    }

    fn fold_one_point(point: &Coords, f: &Fold) -> Coords {
        let (x, y) = *point;
        match f {
            Fold::Vertical(col) => (col - (x - col).abs(), y),
            Fold::Horizontal(row) => (x, row - (y - row).abs()),
        }
    }

    fn show(&self) {
        let (max_x, max_y) = self.points.iter().fold((0, 0), |(max_x, max_y), (x, y)| {
            (max(max_x, *x), max(max_y, *y))
        });
        println!("{}, {}", max_x, max_y);

        for j in 0..max_y + 1 {
            for i in 0..max_x + 1 {
                print!(
                    "{}",
                    if self.points.contains(&(i, j)) {
                        '#'
                    } else {
                        ' '
                    }
                )
            }
            println!();
        }
    }
}

fn part1((paper, foldings): &(Paper, Vec<Fold>)) -> usize {
    paper.fold(&foldings[0]).points.len()
}

fn part2((paper, foldings): &(Paper, Vec<Fold>)) {
    let paper = paper.clone();
    let after_all = foldings.iter().fold(paper, |p, f| p.fold(f));
    after_all.show();
}

fn main() {
    let test = get_input(
        "
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
",
    );
    assert_eq!(17, part1(&test));
    let input = get_input(include_str!("../input"));
    println!("{}", part1(&input));
    part2(&test);
    part2(&input);
}
