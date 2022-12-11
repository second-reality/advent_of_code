pub fn part1(input: String) -> usize {
    let mut register = 1;
    let mut cycle = 1;
    let mut res = 0;
    for line in input.trim().lines() {
        if (cycle - 20) % 40 == 0 {
            res += register * cycle;
        }
        let addx = line.split(' ').last().unwrap().parse::<i32>();
        cycle += if let Ok(increment) = addx {
            if (cycle + 1 - 20) % 40 == 0 {
                res += register * (cycle + 1);
            }
            register += increment;
            2
        } else {
            1
        }
    }
    res as usize
}

fn update_crt(crt: &mut String, cycle: i32, x: i32) {
    let pos = (cycle - 1) % 40;
    let pixel = if pos - 1 <= x && x <= pos + 1 {
        '#'
    } else {
        '.'
    };
    crt.push(pixel);
    if cycle % 40 == 0 {
        crt.push('\n');
    }
}

pub fn part2(input: String) -> usize {
    let mut register = 1;
    let mut cycle = 1;
    let mut res = String::new();
    for line in input.trim().lines() {
        update_crt(&mut res, cycle, register);
        let addx = line.split(' ').last().unwrap().parse::<i32>();
        if let Ok(increment) = addx {
            cycle += 1;
            update_crt(&mut res, cycle, register);
            register += increment;
        }
        cycle += 1;
    }
    println!("{}", res);
    42
}

pub const EXPECTED1: usize = 13_140;
pub const EXPECTED2: usize = 42;
