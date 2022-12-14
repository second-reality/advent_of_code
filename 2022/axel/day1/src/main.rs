type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[cfg(not(test))]
fn main() {
    let input = include_str!("../sample");
    println!("part 1 : {}", resolve(input, 1).unwrap());
    println!("part 2 : {}", resolve(input, 3).unwrap());
}

fn resolve(input: &str, num_elves: usize) -> Result<i32, GenericError> {
    let mut current_cal_count: i32 = 0;

    // sorted vec
    let mut max_cal: Vec<i32> = vec![0; num_elves];

    for line in input.lines() {
        if line.is_empty() {
            if current_cal_count > max_cal[0] {
                max_cal[0] = current_cal_count;
            }

            max_cal.sort_unstable();
            current_cal_count = 0;
        } else {
            current_cal_count += line.parse::<i32>()?;
        }
    }

    // last iteration
    if current_cal_count > max_cal[0] {
        max_cal[0] = current_cal_count;
    }

    Ok(max_cal.into_iter().sum())
}

#[cfg(test)]
mod test {
    use crate::resolve;

    #[test]
    fn example_part1() {
        assert_eq!(
            24_000,
            resolve(
                r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#,
                1
            )
            .unwrap()
        );
    }

    #[test]
    fn example_part2() {
        assert_eq!(
            45_000,
            resolve(
                r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#,
                3
            )
            .unwrap()
        );
    }
}
