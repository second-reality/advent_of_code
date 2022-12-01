pub fn part1(input: String) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|calories| calories.parse::<usize>().unwrap())
                .sum()
        })
        .max()
        .unwrap()
}

pub fn part2(input: String) -> usize {
    let mut total_calories: Vec<usize> = input
        .trim()
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|calories| calories.parse::<usize>().unwrap())
                .sum()
        })
        .collect();

    total_calories.sort_by(|a, b| b.cmp(a));
    total_calories.into_iter().take(3).sum()
}

pub const EXPECTED1: usize = 24_000;
pub const EXPECTED2: usize = 45_000;
