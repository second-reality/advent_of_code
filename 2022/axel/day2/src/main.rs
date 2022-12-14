type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[cfg(not(test))]
fn main() {
    let input = include_str!("../sample");
    println!("part 1 : {}", resolve_part1(input).unwrap());
    println!("part 2 : {}", resolve_part2(input).unwrap());
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Cisor = 3,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum RoundEnd {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

const LOSE_SCORE: u32 = 0;
const DRAW_SCORE: u32 = 3;
const WIN_SCORE: u32 = 6;

fn score(my_choice: Shape, opponent_choice: Shape) -> u32 {
    let mut score = my_choice as u32;

    score += if my_choice == opponent_choice {
        DRAW_SCORE
    } else if my_choice as u32 % 3 == ((opponent_choice as u32) + 1) % 3 {
        WIN_SCORE
    } else {
        LOSE_SCORE
    };

    score
}

fn score_part2(opponent_choice: Shape, objective: RoundEnd) -> u32 {
    let mut score = objective as u32;

    score += if objective == RoundEnd::Draw {
        opponent_choice as u32
    } else if objective == RoundEnd::Win {
        let base = (opponent_choice as u32 + 1) % 3;
        if base == 0 {
            base + 3
        } else {
            base
        }
    } else {
        let base = (opponent_choice as u32 - 1) % 3;
        if base == 0 {
            base + 3
        } else {
            base
        }
    };

    score
}

fn resolve_part1(input: &str) -> Result<u32, GenericError> {
    let mut total_score: u32 = 0;

    for line in input.lines() {
        if let [opponent_shape, my_shape] = line.split(' ').collect::<Vec<&str>>().as_slice() {
            let opponent_choice = match *opponent_shape {
                "A" => Shape::Rock,
                "B" => Shape::Paper,
                "C" => Shape::Cisor,
                _ => unreachable!(),
            };

            let my_choice = match *my_shape {
                "X" => Shape::Rock,
                "Y" => Shape::Paper,
                "Z" => Shape::Cisor,
                _ => unreachable!(),
            };

            total_score += score(my_choice, opponent_choice);
        } else {
            panic!();
        }
    }

    Ok(total_score)
}

fn resolve_part2(input: &str) -> Result<u32, GenericError> {
    let mut total_score: u32 = 0;

    for line in input.lines() {
        if let [opponent_shape, objective] = line.split(' ').collect::<Vec<&str>>().as_slice() {
            let opponent_choice = match *opponent_shape {
                "A" => Shape::Rock,
                "B" => Shape::Paper,
                "C" => Shape::Cisor,
                _ => unreachable!(),
            };

            let objective = match *objective {
                "X" => RoundEnd::Lose,
                "Y" => RoundEnd::Draw,
                "Z" => RoundEnd::Win,
                _ => unreachable!(),
            };

            total_score += score_part2(opponent_choice, objective);
        } else {
            panic!();
        }
    }

    Ok(total_score)
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn example_part1() {
        assert_eq!(
            15,
            resolve_part1(
                r#"A Y
B X
C Z"#
            )
            .unwrap()
        );
    }

    #[test]
    fn example_part2() {
        assert_eq!(
            12,
            resolve_part2(
                r#"A Y
B X
C Z"#
            )
            .unwrap()
        );
    }

    #[test]
    fn test_custom() {
        assert_eq!(resolve_part1("A X").unwrap(), 4);
        assert_eq!(resolve_part1("A Y").unwrap(), 8);
        assert_eq!(resolve_part1("A Z").unwrap(), 3);
        assert_eq!(resolve_part1("B X").unwrap(), 1);
        assert_eq!(resolve_part1("B Y").unwrap(), 5);
        assert_eq!(resolve_part1("B Z").unwrap(), 9);
        assert_eq!(resolve_part1("C X").unwrap(), 7);
        assert_eq!(resolve_part1("C Y").unwrap(), 2);
        assert_eq!(resolve_part1("C Z").unwrap(), 6);

        assert_eq!(resolve_part2("A X").unwrap(), 0 + 3);
        assert_eq!(resolve_part2("A Y").unwrap(), 3 + 1);
        assert_eq!(resolve_part2("A Z").unwrap(), 6 + 2);
        assert_eq!(resolve_part2("B X").unwrap(), 0 + 1);
        assert_eq!(resolve_part2("B Y").unwrap(), 3 + 2);
        assert_eq!(resolve_part2("B Z").unwrap(), 6 + 3);
        assert_eq!(resolve_part2("C X").unwrap(), 0 + 2);
        assert_eq!(resolve_part2("C Y").unwrap(), 3 + 3);
        assert_eq!(resolve_part2("C Z").unwrap(), 6 + 1);
    }
}
