use std::collections::HashMap;
use std::fs;


pub fn solution() -> usize {
    let text = fs::read_to_string("src/day6/input.txt")
        .expect("Something went wrong reading the file");
    let initial_fishes_timer: Vec<usize> = text.trim().split(',').map(|num| num.parse::<usize>().unwrap()).collect();
    let initial_timer_count = initial_fishes_timer.into_iter().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0 as usize) += 1 as usize;
        acc
    });
    let mut occurrences_of_timers:[usize; 9] = [0; 9];
    for (key, value) in initial_timer_count {
        assert!(key < 9);
        occurrences_of_timers[key] = value;
    }

    for _ in 0..256 {
        let tmp0 = occurrences_of_timers[0];
        // decrease all timer of
        for j in 0..8 {
            occurrences_of_timers[j] = occurrences_of_timers[j+1];
        }
        occurrences_of_timers[8] = tmp0;
        occurrences_of_timers[6] += tmp0;
    }


    return occurrences_of_timers.into_iter().sum();
}