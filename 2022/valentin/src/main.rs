use std::env;
use std::fs;
use std::time::Instant;
use val2022aoc::{expected_output_day, get_solution_day, PartSolution};

const NO_COLOR: &str = "\x1b[0m";
const RED: &str = "\x1b[0;31m";
const GREEN: &str = "\x1b[0;32m";
const BLUE: &str = "\x1b[0;34m";
const PURPLE: &str = "\x1b[0;35m";
const HIGH_YELLOW: &str = "\x1b[0;93m";

fn log_str(day: u32, part: u8) -> String {
    format!("{}Day {}, Part {}{}", BLUE, day, part, NO_COLOR)
}

fn time_part(part_func: PartSolution, input: String, day: u32, part: u8) {
    let start = Instant::now();
    let output = part_func(input);
    let duration = start.elapsed();
    println!(
        "{} answer = {}{}{}, took {}{:?}{}",
        log_str(day, part),
        HIGH_YELLOW,
        output,
        NO_COLOR,
        PURPLE,
        duration,
        NO_COLOR
    );
}

fn safe_check(part_func: PartSolution, example: String, expected: usize, day: u32, part: u8) {
    let output = part_func(example);
    if output != expected {
        println!(
            "{} {}Safe check failed (get {}, expected {}){}",
            log_str(day, part),
            RED,
            output,
            expected,
            NO_COLOR
        );
        panic!("Invalid code");
    } else {
        println!(
            "{} {}Safe check passed!{}",
            log_str(day, part),
            GREEN,
            NO_COLOR
        );
    }
}

fn get_string_from(day: u32, is_example: bool) -> String {
    let dir = if is_example { "examples" } else { "inputs" };
    let path = env::current_dir()
        .unwrap()
        .join(dir)
        .join(format!("{:02}.txt", day));
    match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => {
            println!(
                "{}Error file {}/{:02}.txt doesn't exist!{}",
                RED, dir, day, NO_COLOR
            );
            panic!("get_string_from File Not Found")
        }
    }
}

fn exec_and_time_day(day: u32) {
    // get example and input
    let example = get_string_from(day, true);
    let input = get_string_from(day, false);

    // catch the code to test and their expected values
    let (part1, part2) = get_solution_day(day);
    let (expected1, expected2) = expected_output_day(day);

    // test and run part 1
    safe_check(part1, example.clone(), expected1, day, 1);
    time_part(part1, input.clone(), day, 1);

    // test and run part 2
    safe_check(part2, example, expected2, day, 2);
    time_part(part2, input, day, 2);
}

fn main() {
    // Get day from cli argument
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Expected 1 argument for day number (e.g cargo run 12)");
    }

    match args[1].as_str() {
        "all" => {
            for day in 1..26 {
                exec_and_time_day(day);
            }
        }
        arg => {
            let day: u32 = arg
                .parse()
                .expect("Invalid argument should be number (e.g cargo run 12)");
            exec_and_time_day(day);
        }
    }
}
