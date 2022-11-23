use std::cmp::min;
use std::fs;

pub fn number_of_fish(internal_timer: usize, days_left: usize) -> usize {
    if days_left == 0 {
        1
    } else if internal_timer == 0 {
        number_of_fish(8, days_left - 1) + number_of_fish(6, days_left - 1)
    } else {
        let wait_time = min(internal_timer, days_left);
        number_of_fish(internal_timer - wait_time, days_left - wait_time)
    }
}

pub fn solution() -> usize {
    let text = fs::read_to_string("src/day6/input.txt")
        .expect("Something went wrong reading the file");
    let initial_fishes_timer: Vec<usize> = text.trim().split(',').map(|num| num.parse::<usize>().unwrap()).collect();

    return initial_fishes_timer.into_iter().map(|timer| number_of_fish(timer, 80)).sum();
}