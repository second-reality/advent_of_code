use std::env;
use std::fs;
use std::time::Instant;
use val2022aoc::{get_day, PartImplem};

fn time_part(part_func: PartImplem, input: String, day: u32, part: u8) {
    let start = Instant::now();
    let output = part_func(input);
    let duration = start.elapsed();
    println!(
        "Day {}, Part {} answer = {}, took {:?}",
        day, part, output, duration
    );
}

fn get_string_from(day: u32, is_example: bool) -> String {
    let (dir, err_msg) = if is_example {
        ("examples", "Example file missing!")
    } else {
        ("inputs", "Input file missing!")
    };
    let path = env::current_dir()
        .unwrap()
        .join(dir)
        .join(format!("{:02}.txt", day));
    fs::read_to_string(path).expect(err_msg)
}

fn exec_and_time_day(day: u32) {
    // catch the code to test and run
    let ((part1, test1), (part2, test2)) = get_day(day);

    // get example
    let example = get_string_from(day, true);

    // get input
    let input = get_string_from(day, false);

    // testing part 1
    test1(example.clone());
    println!("Day {}, Part 1 test passed !", day);

    // run and time part 1
    time_part(part1, input.clone(), day, 1);

    // test part 2
    test2(example);
    println!("Day {}, Part 2 test passed !", day);

    // run part 2
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
