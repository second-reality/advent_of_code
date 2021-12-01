fn part1(input: &Vec<i32>) {
    let answer = &input
                .windows(2)
                .filter(|i| i[1] > i[0])
                .count();
    println!("part1: {}", answer)
}

fn main() {
    let input_str = include_str!("../input.txt");
    let input: Vec<i32> = input_str
                  .lines()
                  .map(|i| i.parse::<i32>().unwrap())
                  .collect();
    part1(&input);
}
