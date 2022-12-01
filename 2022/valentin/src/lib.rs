pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

pub type PartSolution = fn(String) -> usize;

pub fn get_solution_day(day: u32) -> (PartSolution, PartSolution) {
    match day {
        1 => (day01::part1, day01::part2),
        2 => (day02::part1, day02::part2),
        3 => (day03::part1, day03::part2),
        4 => (day04::part1, day04::part2),
        5 => (day05::part1, day05::part2),
        6 => (day06::part1, day06::part2),
        7 => (day07::part1, day07::part2),
        8 => (day08::part1, day08::part2),
        9 => (day09::part1, day09::part2),
        10 => (day10::part1, day10::part2),
        11 => (day11::part1, day11::part2),
        12 => (day12::part1, day12::part2),
        13 => (day13::part1, day13::part2),
        14 => (day14::part1, day14::part2),
        15 => (day15::part1, day15::part2),
        16 => (day16::part1, day16::part2),
        17 => (day17::part1, day17::part2),
        18 => (day18::part1, day18::part2),
        19 => (day19::part1, day19::part2),
        20 => (day20::part1, day20::part2),
        21 => (day21::part1, day21::part2),
        22 => (day22::part1, day22::part2),
        23 => (day23::part1, day23::part2),
        24 => (day24::part1, day24::part2),
        25 => (day25::part1, day25::part2),
        _ => panic!("Invalid Number of day 1 to 25 expected!"),
    }
}
pub fn expected_output_day(day: u32) -> (usize, usize) {
    match day {
        1 => (day01::EXPECTED1, day01::EXPECTED2),
        2 => (day02::EXPECTED1, day02::EXPECTED2),
        3 => (day03::EXPECTED1, day03::EXPECTED2),
        4 => (day04::EXPECTED1, day04::EXPECTED2),
        5 => (day05::EXPECTED1, day05::EXPECTED2),
        6 => (day06::EXPECTED1, day06::EXPECTED2),
        7 => (day07::EXPECTED1, day07::EXPECTED2),
        8 => (day08::EXPECTED1, day08::EXPECTED2),
        9 => (day09::EXPECTED1, day09::EXPECTED2),
        10 => (day10::EXPECTED1, day10::EXPECTED2),
        11 => (day11::EXPECTED1, day11::EXPECTED2),
        12 => (day12::EXPECTED1, day12::EXPECTED2),
        13 => (day13::EXPECTED1, day13::EXPECTED2),
        14 => (day14::EXPECTED1, day14::EXPECTED2),
        15 => (day15::EXPECTED1, day15::EXPECTED2),
        16 => (day16::EXPECTED1, day16::EXPECTED2),
        17 => (day17::EXPECTED1, day17::EXPECTED2),
        18 => (day18::EXPECTED1, day18::EXPECTED2),
        19 => (day19::EXPECTED1, day19::EXPECTED2),
        20 => (day20::EXPECTED1, day20::EXPECTED2),
        21 => (day21::EXPECTED1, day21::EXPECTED2),
        22 => (day22::EXPECTED1, day22::EXPECTED2),
        23 => (day23::EXPECTED1, day23::EXPECTED2),
        24 => (day24::EXPECTED1, day24::EXPECTED2),
        25 => (day25::EXPECTED1, day25::EXPECTED2),
        _ => panic!("Invalid Number of day 1 to 25 expected!"),
    }
}
