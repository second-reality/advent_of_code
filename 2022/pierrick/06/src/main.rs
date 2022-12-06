use std::collections::HashSet;

type Input = Vec<char>;

fn input(input_str: &str) -> Input {
    input_str.chars().collect()
}

fn find_marker(i: &Input, win_size: usize) -> usize {
    i.windows(win_size)
        .position(|w| HashSet::<char>::from_iter(w.to_owned()).len() == win_size)
        .unwrap()
        + win_size
}

fn part1(i: &Input) -> usize {
    find_marker(i, 4)
}

fn part2(i: &Input) -> usize {
    find_marker(i, 14)
}

fn main() {
    //let test_str = include_str!("../test");
    //let t = input(test_str);
    //assert_eq!(part1(&t), 0);
    //assert_eq!(part2(&t), 0);
    let input_str = include_str!("../input");
    let i = input(input_str);
    assert_eq!(part1(&input("bvwbjplbgvbhsrlpgdmjqwftvncz")), 5);
    println!("{}", part1(&i));
    println!("{}", part2(&i));
}
