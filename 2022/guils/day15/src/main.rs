use itertools::Itertools;

//const INPUT: &str = include_str!("../test.txt"); const Y: i32 = 10; const MAX: i32 = 20;
const INPUT: &str = include_str!("../input.txt");
const Y: i32 = 2000000;
const MAX: i32 = 4000000;

type Coord = (i32, i32);
type Sensor = (Coord, Coord);
type Circle = (Coord, i32);

fn read_input() -> Vec<Sensor> {
    INPUT
        .trim()
        .split('\n')
        .map(|x| {
            x.replace("Sensor at ", "")
                .replace(": closest beacon is at", "")
                .replace(" y=", "")
                .replace("x=", "")
                .split(' ')
                .map(|x| {
                    x.split(',')
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect_vec()
}

fn get_circles(sensors: &[Sensor]) -> Vec<Circle> {
    sensors
        .iter()
        .map(|((sx, sy), (bx, by))| ((*sx, *sy), (bx - sx).abs() + (by - sy).abs()))
        .collect_vec()
}

fn count_positions(circles: &[Circle], y: i32, s: i32, e: i32) -> (i32, i32) {
    let mut count = 0;
    let mut pos = i32::MIN;
    let mut x = i32::MIN;
    for (min_x, mut max_x) in circles
        .iter()
        .map(|((cx, cy), d)| (cx, d - (y - cy).abs()))
        .filter(|(_, dx)| *dx >= 0)
        .map(|(cx, dx)| (cx - dx, cx + dx))
        .sorted_by(|a, b| a.0.cmp(&b.0))
    {
        if pos == i32::MIN {
            if s == -1 {
                pos = min_x;
            } else {
                pos = s;
            }
        }
        if pos < min_x {
            if s != -1 {
                x = pos;
                break;
            }
            pos = min_x;
        }
        if e != -1 && max_x > e {
            max_x = e
        }
        if pos <= max_x {
            count += max_x - pos + 1;
            pos = max_x + 1;
        }
    }
    (count, x)
}

fn count_on_line(sensors: &[Sensor], y: i32) -> i32 {
    let circles = get_circles(sensors);
    let (count, _) = count_positions(&circles, y, -1, -1);
    let count_b = sensors
        .iter()
        .map(|(_, b)| b)
        .filter(|(_, by)| *by == y)
        .sorted()
        .dedup()
        .count() as i32;
    count - count_b
}

fn step1() {
    let sensors = read_input();
    let res = count_on_line(&sensors, Y);
    println!("step1: {res}");
}

fn find_on_square(sensors: &[Sensor], max: i32) -> (i32, i32) {
    let circles = get_circles(sensors);
    for y in 0..=max {
        let (_, x) = count_positions(&circles, y, 0, max);
        if x != i32::MIN {
            return (x, y);
        }
    }
    unreachable!()
}

fn step2() {
    let sensors = read_input();
    let (x, y) = find_on_square(&sensors, MAX);
    let res = x as usize * 4000000 + y as usize;
    println!("step2: {res:?}");
}

fn main() {
    step1();
    step2();
}
