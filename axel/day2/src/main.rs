fn part1(input: &'static str)
{
    let mut depth = 0;
    let mut horizontal_pos = 0;

    for line in input.lines()
    {
        let splitted : Vec<&str>  = line.split_whitespace().collect();
        let cmd = splitted[0];
        let val : i32 = splitted[1].parse::<i32>().unwrap();

        match cmd
        {
            "forward" => horizontal_pos += val,
            "down" => depth += val,
            "up" => depth -= val,
            _ => panic!(),
        }
    }

    println!("depth * horizontal_pos = {}", depth * horizontal_pos);
}

fn part2(input: &'static str)
{
    let mut depth = 0;
    let mut horizontal_pos = 0;
    let mut aim = 0;

    for line in input.lines()
    {
        let splitted : Vec<&str>  = line.split_whitespace().collect();
        let cmd = splitted[0];
        let val : i32 = splitted[1].parse::<i32>().unwrap();

        match cmd
        {
            "forward" => { horizontal_pos += val; depth += val * aim },
            "down" => aim += val,
            "up" => aim -= val,
            _ => panic!(),
        }
    }

    println!("depth * horizontal_pos = {}", depth * horizontal_pos);
}

fn main() {
    let input = include_str!("../input");
    part1(input);
    part2(input);
}
