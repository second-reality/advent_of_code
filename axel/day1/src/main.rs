fn main() {
    let input = include_str!("../sample");

    let mut increase_cmpt : i32 = 0;
    let mut pred_floor_depth = i32::MAX;

    for line in input.lines() {
        let floor_depth = line.parse::<i32>().unwrap();

        if floor_depth > pred_floor_depth {
            increase_cmpt += 1;
        }
        pred_floor_depth = floor_depth;
    }

    println!("{}", increase_cmpt);
}

