type Input = Vec<Vec<i32>>;

fn part1(input: &Input) -> i32 {
    *total_calories_sorted(input).last().unwrap()
}

fn part2(input: &Input) -> i32 {
    total_calories_sorted(input).iter().rev().take(3).sum()
}

fn total_calories_sorted(input: &Input) -> Vec<i32> {
    let mut totals: Vec<i32> = input.iter().map(|list| list.iter().sum()).collect();
    totals.sort();
    totals
}

fn input() -> Input {
    let input_str = include_str!("../input");
    input_str
        .split("\n\n")
        .map(|s| {
            s.lines()
                .map(|calorie| calorie.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn main() {
    let i = input();
    println!("{}", part1(&i));
    println!("{}", part2(&i));
}
