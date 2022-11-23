use std::fs;

pub fn solution() -> usize {
    let text = fs::read_to_string("src/day7/input.txt")
        .expect("Something went wrong reading the file");
    let initial_crab_positions: Vec<isize> = text.trim().split(',').map(|num| num.parse::<isize>().unwrap()).collect();
    let min_pos = *initial_crab_positions.iter().min().unwrap();
    let max_pos = *initial_crab_positions.iter().max().unwrap();
    println!("min pos: {}", min_pos);
    println!("max pos: {}", max_pos);
    let mut better_pos = min_pos;
    let mut min_fuel_consumed:usize = 9999999999999;
    for pos in min_pos..(max_pos+1) {
        let fuel_cur = initial_crab_positions.iter().map(|p| (*p - pos).abs() as usize).sum();
        if fuel_cur < min_fuel_consumed {
            min_fuel_consumed = fuel_cur;
            better_pos = pos;
        }
    }
    println!("better position: {}", better_pos);
    return min_fuel_consumed;
}