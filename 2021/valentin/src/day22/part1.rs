use std::collections::HashSet;
use std::fs;

pub fn parse_line(line: &str) -> (bool, i32, i32, i32, i32, i32, i32) {
    let split: Vec<&str> = line.split(' ').collect();
    let on = split[0] == "on";
    let split: Vec<&str> = split[1].split(',').collect();
    let split_x: Vec<i32> = split[0].split("..")
        .map(|s| s.replace("x=", "").parse::<i32>().unwrap())
        .collect();
    let split_y: Vec<i32> = split[1].split("..")
        .map(|s| s.replace("y=", "").parse::<i32>().unwrap())
        .collect();
    let split_z: Vec<i32> = split[2].split("..")
        .map(|s| s.replace("z=", "").parse::<i32>().unwrap())
        .collect();
    (on, split_x[0], split_x[1], split_y[0], split_y[1], split_z[0], split_z[1])
}

pub fn solution() -> usize {
    let text = fs::read_to_string("src/day22/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim().split('\n').collect();
    let mut on_points: HashSet<(i32, i32, i32)> = HashSet::new();

    for line in lines {
        let (on, x_min, x_max, y_min, y_max, z_min, z_max) = parse_line(line);
        if x_min < -50 || x_max > 50 || y_min < -50 || y_max > 50 || z_min < -50 || z_max > 50 {
            continue;
        }

        for x in x_min..(x_max + 1) {
            for y in y_min..(y_max + 1) {
                for z in z_min..(z_max + 1) {
                    if on {
                        on_points.insert((x, y, z));
                    } else {
                        on_points.remove(&(x, y, z));
                    }
                }
            }
        }
    }

    on_points.len()
}