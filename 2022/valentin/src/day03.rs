pub fn part1(input: String) -> usize {
    input
        .trim()
        .split('\n')
        .map(|line| {
            let (compart1, compart2) = line.split_at(line.len() / 2);
            compart2
                .chars()
                .find(|item| compart1.contains(*item))
                .unwrap()
        })
        .map(|item| {
            if item.is_ascii_lowercase() {
                // a is 97 and we map a -> 1, b -> 2 etc.
                (item as usize) - 96
            } else {
                // should be ascii uppercase starting at ascii 65
                // same as before but A -> 27, B -> 27 etc.
                (item as usize) - 64 + 26
            }
        })
        .sum()
}

pub fn part2(input: String) -> usize {
    let elfs: Vec<&str> = input.trim().split('\n').collect();
    elfs.iter()
        .enumerate()
        .step_by(3)
        .map(|(i, elf1)| {
            let elf2 = elfs[i + 1];
            let elf3 = elfs[i + 2];
            elf1.chars()
                .find(|item| elf2.contains(*item) && elf3.contains(*item))
                .unwrap()
        })
        .map(|item| {
            if item.is_ascii_lowercase() {
                // a is 97 and we map a -> 1, b -> 2 etc.
                (item as usize) - 96
            } else {
                // should be ascii uppercase starting at ascii 65
                // same as before but A -> 27, B -> 27 etc.
                (item as usize) - 64 + 26
            }
        })
        .sum()
}

pub const EXPECTED1: usize = 157;
pub const EXPECTED2: usize = 70;
