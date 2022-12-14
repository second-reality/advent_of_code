type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[cfg(not(test))]
fn main() {
    let input = include_str!("../sample");
    println!("part 1 : {}", resolve(input).unwrap());
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Cisor = 3,
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

fn resolve(input: &str) -> Result<u32, GenericError> {
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

#[cfg(test)]
mod test {
    use crate::resolve;

    #[test]
    fn example_part1() {
        assert_eq!(
            15,
            resolve(
                r#"A Y
B X
C Z"#
            )
            .unwrap()
        );
    }

    #[test]
    fn test_custom() {
        assert_eq!(resolve("A X").unwrap(), 4);
        assert_eq!(resolve("A Y").unwrap(), 8);
        assert_eq!(resolve("A Z").unwrap(), 3);
        assert_eq!(resolve("B X").unwrap(), 1);
        assert_eq!(resolve("B Y").unwrap(), 5);
        assert_eq!(resolve("B Z").unwrap(), 9);
        assert_eq!(resolve("C X").unwrap(), 7);
        assert_eq!(resolve("C Y").unwrap(), 2);
        assert_eq!(resolve("C Z").unwrap(), 6);
    }
}
