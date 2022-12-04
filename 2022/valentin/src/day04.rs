use std::cmp::{max, min};

pub fn part1(input: String) -> usize {
    input
        .trim()
        .split('\n')
        .map(|line| {
            let mut ranges = line.split(',').map(|elf| {
                let mut range = elf.split('-').map(|id| id.parse::<u8>().unwrap());
                let id_min = range.next().unwrap();
                let id_max = range.next().unwrap();
                (id_min, id_max)
            });
            (ranges.next().unwrap(), ranges.next().unwrap())
        })
        .filter(|((min1, max1), (min2, max2))| {
            (min1 <= min2 && max1 >= max2) || (min2 <= min1 && max2 >= max1)
        })
        .count()
}

pub fn part2(input: String) -> usize {
    input
        .trim()
        .split('\n')
        .map(|line| {
            let mut ranges = line.split(',').map(|elf| {
                let mut range = elf.split('-').map(|id| id.parse::<u8>().unwrap());
                let id_min = range.next().unwrap();
                let id_max = range.next().unwrap();
                (id_min, id_max)
            });
            (ranges.next().unwrap(), ranges.next().unwrap())
        })
        .filter(|((min1, max1), (min2, max2))| max(min1, min2) <= min(max1, max2))
        .count()
}

pub const EXPECTED1: usize = 2;
pub const EXPECTED2: usize = 4;
