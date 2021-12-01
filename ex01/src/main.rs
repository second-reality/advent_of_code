fn part1(input: &[i32]) -> usize {
    input.windows(2).filter(|i| i[1] > i[0]).count()
}

fn part2(input: &[i32]) -> usize {
    let all_sums: Vec<i32> = input.windows(3).map(|i| i[0] + i[1] + i[2]).collect();
    part1(&all_sums)
}

fn main() {
    let input_str = include_str!("../input.txt");
    let input: Vec<i32> = input_str
        .lines()
        .map(|i| i.parse::<i32>().unwrap())
        .collect();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
