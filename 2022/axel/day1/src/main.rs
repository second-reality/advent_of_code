type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[cfg(not(test))]
fn main() {
    let input = include_str!("../sample");
    println!("part 1 : {}", resolve(input).unwrap());
}

fn resolve(input: &str) -> Result<i32, GenericError>
{
    let mut current_cal_count : i32 = 0;
    let mut max_cal : i32 = 0;

    for line in input.lines() {
        if line.is_empty() {
            if current_cal_count > max_cal {
                max_cal = current_cal_count;
            }

            current_cal_count = 0;
        }
        else {
            current_cal_count += line.parse::<i32>()?;
        }
    }

    Ok(max_cal)
}

#[cfg(test)]
mod test
{
    use crate::resolve;

    #[test]
    fn test_example()
    {
        assert_eq!(24_000,
                   resolve(r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#).unwrap());

    }
}
