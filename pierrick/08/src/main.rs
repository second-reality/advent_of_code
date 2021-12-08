fn get_line(s: &str) -> Vec<String> {
    let mut line = s.split(" | ");
    let right = line.nth(1).unwrap();
    right.split_whitespace().map(String::from).collect()
}

fn get_input(s: &str) -> Vec<Vec<String>> {
    s.lines().map(get_line).collect()
}

// 0 -> 6 segments
// 1 -> 2 segments (unique)
// 2 -> 5 segments
// 3 -> 5 segments
// 4 -> 4 segments (unique)
// 5 -> 5 segments
// 6 -> 6 segments
// 7 -> 3 segments (unique)
// 8 -> 7 segments (unique)
// 9 -> 6 segments

fn is_unique_digit(s: &str) -> bool {
    let len = s.len();
    len == 2 || len == 4 || len == 3 || len == 7
}

fn unique_digits(words: &[Vec<String>]) -> usize {
    words
        .iter()
        .flatten()
        .filter(|s| is_unique_digit(s))
        .count()
}

fn main() {
    let input = get_input(include_str!("../input.txt"));
    let test = get_input(include_str!("../test_input.txt"));
    println!("test {}", unique_digits(&test));
    println!("{}", unique_digits(&input));
}
