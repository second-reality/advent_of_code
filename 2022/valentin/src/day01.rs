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

    total_calories.sort();
    total_calories.reverse();
    total_calories.into_iter().take(3).sum()
}

pub fn test1(example: String) {
    assert_eq!(part1(example), 24000);
}

pub fn test2(example: String) {
    assert_eq!(part2(example), 45000);
}
