use std::cmp::*;

#[derive(Debug, PartialEq)]
struct Wind {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Wind {
    fn new(s: &str) -> Self {
        let coords: Vec<i32> = s
            .split(" -> ")
            .map(|coord| coord.split(','))
            .flatten()
            .map(|x| x.parse().unwrap())
            .collect();
        Wind {
            x1: coords[0],
            y1: coords[1],
            x2: coords[2],
            y2: coords[3],
        }
    }
}

#[test]
fn try_parse() {
    assert_eq!(
        Wind::new("1,2 -> 3,4"),
        Wind {
            x1: 1,
            y1: 2,
            x2: 3,
            y2: 4
        }
    );
}

struct Ocean {
    grid: Vec<i32>,
    width: usize,
}

impl Ocean {
    fn new(width: usize) -> Self {
        Ocean {
            grid: vec![0; width * width],
            width,
        }
    }

    fn mark_point(&mut self, x: i32, y: i32) {
        self.grid[(y as usize * self.width + x as usize)] += 1;
    }

    fn blow_wind_part1(&mut self, w: &Wind) {
        let x_from = min(w.x1, w.x2);
        let x_to = max(w.x1, w.x2);
        let y_from = min(w.y1, w.y2);
        let y_to = max(w.y1, w.y2);

        if x_from == x_to {
            (y_from..y_to + 1).for_each(|y| self.mark_point(x_from, y));
        }

        if y_from == y_to {
            (x_from..x_to + 1).for_each(|x| self.mark_point(x, y_from));
        }
    }

    fn blow_wind_part2(&mut self, w: &Wind) {
        let x_from = min(w.x1, w.x2);
        let x_to = max(w.x1, w.x2);
        let y_from = min(w.y1, w.y2);
        let y_to = max(w.y1, w.y2);

        if x_to - x_from != y_to - y_from {
            return;
        }

        let num_steps = x_to - x_from;
        let x_step = 1 + -2 * (w.x2 < w.x1) as i32;
        let y_step = 1 + -2 * (w.y2 < w.y1) as i32;

        (0..num_steps + 1).for_each(|s| self.mark_point(w.x1 + s * x_step, w.y1 + s * y_step));
    }

    fn answer(&self) -> usize {
        self.grid.iter().filter(|&x| *x >= 2).count()
    }
}

fn get_input() -> Vec<Wind> {
    include_str!("../input.txt")
        .lines()
        .map(Wind::new)
        .collect()
}

fn blow_winds(blower: fn(&mut Ocean, &Wind)) -> usize {
    let mut ocean = Ocean::new(1000);
    for w in get_input() {
        blower(&mut ocean, &w);
    }
    ocean.answer()
}

fn main() {
    println!("{}", blow_winds(|ocean, wind| ocean.blow_wind_part1(wind)));
    println!(
        "{}",
        blow_winds(|ocean, wind| {
            ocean.blow_wind_part1(wind);
            ocean.blow_wind_part2(wind)
        })
    );
}
