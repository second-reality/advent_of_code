use std::fs;
use std::ops::Range;

pub fn parse_input_ranges(text: String) -> (Range<i32>, Range<i32>) {
    let text = &text.trim()[13..];
    let split: Vec<&str> = text.split(", ").collect();
    let tmp: Vec<i32> = split[0][2..].split("..").map(|x| x.parse::<i32>().unwrap()).collect();
    let x_range = tmp[0]..tmp[1];
    let tmp: Vec<i32> = split[1][2..].split("..").map(|y| y.parse::<i32>().unwrap()).collect();
    let y_range = tmp[0]..tmp[1];
    (x_range, y_range)
}

pub fn get_max_high_during_yeet(mut vx: i32, mut vy: i32, x_dest: &Range<i32>, y_dest: &Range<i32>) -> i32 {
    let (mut x, mut y) = (0, 0);
    let mut y_max = 0;
    while (!x_dest.contains(&x)) || (!y_dest.contains(&y)) {
        // update position
        x += vx;
        y += vy;
        if y > y_max {
            y_max = y;
        }
        // update speed
        vx -= if vx > 0 { 1 } else { 0 };
        vy -= 1;
        // if we skipped end area return error value
        if y < y_dest.start || x > x_dest.end {
            return -1;
        }
    }
    y_max
}

pub fn find_best_initial_speed(x_dest: &Range<i32>, y_dest: &Range<i32>) -> (i32, i32, i32) {
    // formula obtained when searching the limit of x in x_dest
    let vx_best = (1 + (f32::sqrt((1 + 8 * ((x_dest.end + x_dest.start) / 2)) as f32) as i32)) / 2;
    let mut vy_best = 0;
    let x_dest_include_end = x_dest.start..(x_dest.end + 1);
    let y_dest_include_end = y_dest.start..(y_dest.end + 1);
    let mut best_score = 0;
    let mut vy = 1;
    let mut not_skipping = true;
    while not_skipping {
        let score = get_max_high_during_yeet(vx_best, vy, &x_dest_include_end, &y_dest_include_end);
        not_skipping = score != -1;
        if score > best_score {
            best_score = score;
            vy_best = vy;
            vy *= 2;
        }
    }
    let last_possible_vy = vy * 2;
    vy = vy_best;
    while vy < last_possible_vy {
        vy += 1;
        let score = get_max_high_during_yeet(vx_best, vy, &x_dest_include_end, &y_dest_include_end);
        if score > best_score {
            best_score = score;
            vy_best = vy;
        }
    }

    (vx_best, vy_best, best_score)
}

pub fn solution() -> i32 {
    let text = fs::read_to_string("src/day17/input.txt")
        .expect("Something went wrong reading the file");
    let (x_range, y_range) = parse_input_ranges(text);
    let (vx_best, vy_best, score) = find_best_initial_speed(&x_range, &y_range);
    println!("best speed: x={}, y={}, max high={}", vx_best, vy_best, score);
    score
}