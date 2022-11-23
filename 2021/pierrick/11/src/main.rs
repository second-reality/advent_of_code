use itertools::*;

struct Cave {
    width: i32,
    height: i32,
    data: Vec<u32>,
}

impl Cave {
    fn step(&mut self) -> usize {
        iproduct!(0..self.width, 0..self.height).for_each(|(x, y)| self.increase(x, y));
        let num_flashed = self.data.iter().filter(|&x| *x > 9).count();
        self.data.iter_mut().for_each(|x| {
            if *x > 9 {
                *x = 0;
            }
        });
        num_flashed
    }

    fn increase(&mut self, x: i32, y: i32) {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return;
        }

        let index = (y * self.width + x) as usize;
        let val = self.data.get_mut(index).unwrap();
        *val += 1;
        if *val == 10 {
            // left
            self.increase(x - 1, y - 1);
            self.increase(x - 1, y);
            self.increase(x - 1, y + 1);
            // right
            self.increase(x + 1, y - 1);
            self.increase(x + 1, y);
            self.increase(x + 1, y + 1);
            // up
            self.increase(x, y + 1);
            // down
            self.increase(x, y - 1);
        }
    }
}

fn get_input(s: &str) -> Cave {
    let width = s.lines().next().unwrap().chars().count() as i32;
    let height = s.lines().count() as i32;
    let data = s
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    Cave {
        width,
        height,
        data,
    }
}

fn part1(input: &str) -> usize {
    let mut cave = get_input(input);
    (0..100).map(|_| cave.step()).sum()
}

fn part2(input: &str) -> usize {
    let mut cave = get_input(input);
    let expected = (cave.width * cave.height) as usize;
    (1..usize::MAX).take_while(|_| cave.step() != expected).last().unwrap() + 1
}

fn main() {
    let test = include_str!("../test");
    let input = include_str!("../input");
    println!("test {}", part1(test));
    println!("{}", part1(input));
    println!("test {}", part2(test));
    println!("{}", part2(input));
}
