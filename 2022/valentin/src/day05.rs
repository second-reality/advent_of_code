type Instructions = Vec<(usize, usize, usize)>;
type CratesStacks = Vec<Vec<char>>;

fn parser(input: String) -> (CratesStacks, Instructions) {
    let mut input = input.trim_end().split("\n\n");
    let (stacks_str, instructions) = (input.next().unwrap(), input.next().unwrap());
    let mut stacks_str = stacks_str.split('\n').rev();
    let n: usize = stacks_str
        .next()
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let mut stacks: CratesStacks = (0..n).map(|_| Vec::new()).collect();
    for crates in stacks_str {
        crates
            .chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter_map(|(i, c)| if c == ' ' { None } else { Some((i, c)) })
            .for_each(|(i, c)| stacks[i].push(c))
    }
    let instructions: Instructions = instructions
        .split('\n')
        .map(|instruction| {
            let mut digits = instruction
                .split(' ')
                .filter_map(|s| s.parse::<usize>().ok());
            let moves = digits.next().unwrap();
            let from = digits.next().unwrap() - 1;
            let to = digits.next().unwrap() - 1;
            (moves, from, to)
        })
        .collect();
    (stacks, instructions)
}
pub fn part1(input: String) -> usize {
    let (mut stacks, instructions) = parser(input);
    for (moves, from, to) in instructions {
        for _ in 0..moves {
            let cra = stacks[from].pop().unwrap();
            stacks[to].push(cra);
        }
    }
    let answer = stacks
        .into_iter()
        .map(|v| v.last().unwrap().to_string())
        .fold(String::new(), |acc, cur| acc + &cur);
    println!("answer = {}", answer);
    answer.chars().map(|c| c as usize).sum()
}

pub fn part2(input: String) -> usize {
    let (mut stacks, instructions) = parser(input);
    for (moves, from, to) in instructions {
        let i = stacks[to].len();
        for _ in 0..moves {
            let cra = stacks[from].pop().unwrap();
            stacks[to].insert(i, cra);
        }
    }
    let answer = stacks
        .into_iter()
        .map(|v| v.last().unwrap().to_string())
        .fold(String::new(), |acc, cur| acc + &cur);
    println!("answer = {}", answer);
    answer.chars().map(|c| c as usize).sum()
}

pub const EXPECTED1: usize = ('C' as usize) + ('M' as usize) + ('Z' as usize);
pub const EXPECTED2: usize = ('M' as usize) + ('C' as usize) + ('D' as usize);
