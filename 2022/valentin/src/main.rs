use std::env;
use std::fs;
use std::time::Instant;
use val2022aoc::get_day;

fn exec_and_time_day(day: u32) {
    // catch the code to test and run
    let ((part1, test1), (part2, test2)) = get_day(day);

    // get example input to do quick safe checks
    let path_example = env::current_dir()
        .unwrap()
        .join("examples")
        .join(format!("{:02}.txt", day));
    let example = fs::read_to_string(path_example).expect("Example file not found");
    // testing part 1
    println!("Testing Part 1...");
    test1(example.clone());
    println!("Part 1 test is ok !");

    // get input
    let path_input = env::current_dir()
        .unwrap()
        .join("inputs")
        .join(format!("{:02}.txt", day));
    let input = fs::read_to_string(path_input).expect("Input file not found");
    // run part 1
    println!("Running Part 1...");
    let start = Instant::now();
    let answer = part1(input.clone());
    let duration = start.elapsed();
    println!("Part 1 answer is {}, takes {:?}", answer, duration);

    // test part 2
    println!("Testing Part 2...");
    test2(example);
    println!("Part 2 test is ok !");

    // run part 2
    println!("Running Part 2...");
    let start = Instant::now();
    let answer = part2(input);
    let duration = start.elapsed();
    println!("Part 2 answer is {}, takes {:?}", answer, duration);
}

fn main() {
    // Get day from cli argument
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Expected 1 argument for day number (1-25)");
        return;
    }

    match args[1].as_str() {
        "all" => {
            for day in 1..26 {
                exec_and_time_day(day);
            }
        }
        arg => {
            let day: u32 = arg.parse().unwrap();
            exec_and_time_day(day);
        }
    }
}
