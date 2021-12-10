#[derive(Debug, PartialEq)]
enum ParsingError {
    BadClosing(char),
    UnknownChar(char),
    Incomplete(Vec<char>),
}

fn close_token(c: char, char_stack: &mut Vec<char>) -> Result<(), ParsingError> {
    match char_stack.last() {
        Some(expected) => {
            if c != *expected {
                Err(ParsingError::BadClosing(c))
            } else {
                char_stack.pop();
                Ok(())
            }
        }
        None => Err(ParsingError::BadClosing(c)),
    }
}

fn parse_line(s: &str) -> Result<(), ParsingError> {
    let mut char_stack = vec![];

    for c in s.chars().into_iter() {
        match c {
            '(' => char_stack.push(')'),
            '{' => char_stack.push('}'),
            '[' => char_stack.push(']'),
            '<' => char_stack.push('>'),
            ')' | ']' | '}' | '>' => close_token(c, &mut char_stack)?,
            _ => return Err(ParsingError::UnknownChar(c)),
        }
    }

    if char_stack.is_empty() {
        Ok(())
    } else {
        let missing: Vec<char> = char_stack.into_iter().rev().collect();
        Err(ParsingError::Incomplete(missing))
    }
}

#[test]
fn test_one_line() {
    assert_eq!(Ok(()), parse_line("<>"));
    assert_eq!(Ok(()), parse_line("<()>"));
    assert_eq!(Ok(()), parse_line("<(())>"));
    assert_eq!(
        Err(ParsingError::Incomplete(vec!['>'])),
        parse_line("<(())")
    );
    assert_eq!(Err(ParsingError::BadClosing('>')), parse_line(">"));
    assert_eq!(Err(ParsingError::BadClosing('>')), parse_line("<([>"));
}

fn score_for_one_line_part1(line: &str) -> usize {
    match parse_line(line) {
        Ok(()) | Err(ParsingError::Incomplete(_)) => 0,
        Err(ParsingError::UnknownChar(_)) => panic!("unknown char"),
        Err(ParsingError::BadClosing(c)) => match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("bad closing char"),
        },
    }
}

fn score_for_one_line_part2(line: &str) -> usize {
    if let Err(ParsingError::Incomplete(missing)) = parse_line(line) {
        missing.iter().fold(0, |acc, c| {
            acc * 5
                + (match c {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => panic!("bad closing"),
                })
        })
    } else {
        0
    }
}

fn part1(lines: &[&str]) -> usize {
    lines.iter().map(|l| score_for_one_line_part1(l)).sum()
}

fn part2(lines: &[&str]) -> usize {
    let mut scores: Vec<usize> = lines
        .iter()
        .map(|l| score_for_one_line_part2(l))
        .filter(|s| *s > 0)
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn main() {
    let test: Vec<&str> = include_str!("../test").lines().collect();
    let input: Vec<&str> = include_str!("../input").lines().collect();
    println!("test {}", part1(&test));
    println!("{}", part1(&input));
    println!("test {}", part2(&test));
    println!("{}", part2(&input));
}
