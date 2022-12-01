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

pub type PartImplem = fn(String) -> usize;
pub type TestPart = fn(String);
pub type PartSolution = (PartImplem, TestPart);
pub type DaySolution = (PartSolution, PartSolution);

pub fn get_day(day: u32) -> DaySolution {
    match day {
        1 => ((day01::part1, day01::test1), (day01::part2, day01::test2)),
        2 => ((day02::part1, day02::test1), (day02::part2, day02::test2)),
        3 => ((day03::part1, day03::test1), (day03::part2, day03::test2)),
        4 => ((day04::part1, day04::test1), (day04::part2, day04::test2)),
        5 => ((day05::part1, day05::test1), (day05::part2, day05::test2)),
        6 => ((day06::part1, day06::test1), (day06::part2, day06::test2)),
        7 => ((day07::part1, day07::test1), (day07::part2, day07::test2)),
        8 => ((day08::part1, day08::test1), (day08::part2, day08::test2)),
        9 => ((day09::part1, day09::test1), (day09::part2, day09::test2)),
        10 => ((day10::part1, day10::test1), (day10::part2, day10::test2)),
        11 => ((day11::part1, day11::test1), (day11::part2, day11::test2)),
        12 => ((day12::part1, day12::test1), (day12::part2, day12::test2)),
        13 => ((day13::part1, day13::test1), (day13::part2, day13::test2)),
        14 => ((day14::part1, day14::test1), (day14::part2, day14::test2)),
        15 => ((day15::part1, day15::test1), (day15::part2, day15::test2)),
        16 => ((day16::part1, day16::test1), (day16::part2, day16::test2)),
        17 => ((day17::part1, day17::test1), (day17::part2, day17::test2)),
        18 => ((day18::part1, day18::test1), (day18::part2, day18::test2)),
        19 => ((day19::part1, day19::test1), (day19::part2, day19::test2)),
        20 => ((day20::part1, day20::test1), (day20::part2, day20::test2)),
        21 => ((day21::part1, day21::test1), (day21::part2, day21::test2)),
        22 => ((day22::part1, day22::test1), (day22::part2, day22::test2)),
        23 => ((day23::part1, day23::test1), (day23::part2, day23::test2)),
        24 => ((day24::part1, day24::test1), (day24::part2, day24::test2)),
        25 => ((day25::part1, day25::test1), (day25::part2, day25::test2)),
        _ => panic!("Invalid Number of day 1 to 25 expected!"),
    }
}
