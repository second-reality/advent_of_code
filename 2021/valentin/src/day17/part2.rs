use std::cmp::{max, min};
use std::fs;
use std::ops::Range;
use crate::day17::part1::{find_best_initial_speed, get_max_high_during_yeet, parse_input_ranges};

fn valid_speeds(x_dest: Range<i32>, y_dest: Range<i32>, vx_min:i32, vx_max:i32, vy_min:i32, vy_max:i32) -> Vec<(i32, i32)> {
    let x_dest_include_end = x_dest.start..(x_dest.end + 1);
    let y_dest_include_end = y_dest.start..(y_dest.end + 1);

    let mut res:Vec<(i32, i32)> = Vec::new();
    for vx in vx_min..vx_max {
        for vy in vy_min..vy_max {
            if get_max_high_during_yeet(vx, vy, &x_dest_include_end, &y_dest_include_end) != -1 {
                res.push((vx, vy))
            }
        }
    }
    res
}

pub fn solution() -> usize {
    let text = fs::read_to_string("src/day17/input.txt")
        .expect("Something went wrong reading the file");
    let (x_range, y_range) = parse_input_ranges(text);


    let vx_min = min((1 + (f32::sqrt((1 + 8 * x_range.start) as f32) as i32)) / 2, x_range.start);
    let vx_max = max((1 + (f32::sqrt((1 + 8 * x_range.end) as f32) as i32)) / 2, x_range.end);
    println!("vx min: {}, max:{}", vx_min, vx_max);

    let vy_min = y_range.start;
    let (_, vy_max, _) = find_best_initial_speed(&x_range, &y_range);
    println!("vy min: {}, max:{}", vy_min, vy_max);

    let vx_max= vx_max + 1;
    let vy_max= vy_max + 1;

    let init_speeds = valid_speeds(x_range, y_range, vx_min, vx_max, vy_min, vy_max);
    println!("{:?}", init_speeds);
    init_speeds.len()
}