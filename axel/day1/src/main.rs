
fn main() {
    let input = include_str!("../sample");

    let mut increase_cmpt : i32 = 0;
    let mut pred_floor_depth = i32::MAX;

    // Part 1
    for line in input.lines() {
        let floor_depth = line.parse::<i32>().unwrap();

        if floor_depth > pred_floor_depth {
            increase_cmpt += 1;
        }
        pred_floor_depth = floor_depth;
    }

    let mut increase_cmpt_part2 : i32 = 0;
    let mut pred_floor_depth_part2 = i32::MAX;
    // Part 2
    for window in input.lines().collect::<Vec<&str>>().windows(3)
    {
        let sum : i32 = window.iter().map(|s| s.parse::<i32>().unwrap()).sum();
        if sum > pred_floor_depth_part2
        {
            increase_cmpt_part2 += 1;
        }
        pred_floor_depth_part2 = sum;
    }


    println!("part 1 : {}", increase_cmpt);
    println!("part 2 : {}", increase_cmpt_part2);
}

