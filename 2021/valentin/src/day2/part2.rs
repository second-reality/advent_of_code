use std::fs;

pub fn solution() -> i32 {
    let text = fs::read_to_string("src/day2/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim().split('\n').collect();
    let mut horizontal:i32 = 0;
    let mut depth:i32 = 0;
    let mut aim:i32 = 0;
    for line in lines {
        let spitted:Vec<&str> = line.split(' ').collect();
        let instruction = spitted[0];
        let number = spitted[1].parse::<i32>().expect("no number given after instruction");
        match instruction {
            "forward" => {
                horizontal += number;
                depth += aim * number;
            }
            "down" => {
                aim += number;
            }
            "up" => {
                aim -= number;
            }
            _ => {
                println!("Error instruction unknown!");
            }
        }
    }

    return horizontal * depth ;
}