use std::collections::HashSet;
use std::fs;

pub fn parse_scanners_data(lines:Vec<&str>) -> Vec<Vec<(i32, i32, i32)>> {
    let mut res:Vec<Vec<(i32, i32, i32)>> = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        if lines[i].contains("scanner") {
            i += 1;
            let mut beacons:Vec<(i32, i32, i32)> = Vec::new();
            while i < lines.len() && !lines[i].is_empty() {
                let split:Vec<i32>= lines[i].split(",").map(|s| s.parse::<i32>().unwrap()).collect();
                beacons.push((split[0], split[1], split[2]));
                i += 1;
            }
            res.push(beacons);
        } else if lines[i].is_empty() {
            i += 1;
        }
    }
    res
}

fn apply_config(mut z_rotation:i32, mut x_rotation:i32, coords:(i32, i32, i32)) -> (i32, i32, i32) {
    z_rotation = ((z_rotation % 360) + 360) % 360;
    x_rotation = ((x_rotation % 360) + 360) % 360;
    let (x, y, z) = coords;

    let (new_x, new_y) = match z_rotation {
        0 => (x, y),
        90 => (y, -x),
        180 => (-x, -y),
        270 => (y, x),
        _ => {
            println!("WTF angle not multiple of 90 degrees");
            (-1, -1)
        }
    };
    (new_x, new_y, new_z)
}

// fn try_all_configurations(test:&Vec<(i32, i32, i32)>, known:&HashSet<(i32, i32, i32)>) -> (i32, Vec<(i32, i32, i32)>) {
//
//     for i in 0..24 {
//
//     }
// }

pub fn solution() -> usize {
    let text = fs::read_to_string("src/day19/example.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.split('\n').collect();
    let scanners_datas = parse_scanners_data(lines);
    println!("{:?}", scanners_datas);
    42
}