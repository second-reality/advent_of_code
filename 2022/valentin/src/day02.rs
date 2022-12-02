pub fn part1(input: String) -> usize {
    input
        .trim()
        .split('\n')
        .map(|line| {
            let mut tmp = line.split(' ');
            let opponent = tmp.next().unwrap();
            let you = tmp.next().unwrap();
            let shape = match you {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => panic!("Unexpected input for player"),
            };
            let outcome = match (you, opponent) {
                ("X", "B") | ("Y", "C") | ("Z", "A") => 0,
                ("X", "A") | ("Y", "B") | ("Z", "C") => 3,
                ("X", "C") | ("Y", "A") | ("Z", "B") => 6,
                _ => panic!("Unexpected input for both players"),
            };
            shape + outcome
        })
        .sum()
}

pub fn part2(input: String) -> usize {
    input
        .trim()
        .split('\n')
        .map(|line| {
            let mut tmp = line.split(' ');
            let opponent = tmp.next().unwrap();
            let you = tmp.next().unwrap();
            let outcome = match you {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => panic!("Unexpected input for player"),
            };
            let shape = match (you, opponent) {
                ("X", "B") | ("Y", "A") | ("Z", "C") => 1,
                ("X", "C") | ("Y", "B") | ("Z", "A") => 2,
                ("X", "A") | ("Y", "C") | ("Z", "B") => 3,
                _ => panic!("Unexpected input for both players"),
            };
            shape + outcome
        })
        .sum()
}

pub const EXPECTED1: usize = 15;
pub const EXPECTED2: usize = 12;
