extern crate rayon;
use rayon::prelude::*;

const NUM_OF_DAYS : i64 = 256;

fn main() {
    let input = include_str!("../input").trim();
    let initial_number_of_fishes = input.split(",").count() as i64;
    let total = input.split(",")
                     .map(|c| c.parse::<i64>().unwrap())
                     .map(|i| number_of_fishes(i, NUM_OF_DAYS))
                     .sum::<i64>();
    println!("{:#?}", total + initial_number_of_fishes);
}

fn number_of_fishes(fish_timer: i64, days_remaining: i64) -> i64 {
    if fish_timer >= days_remaining {
        0
    } else {
        let (a,b) = rayon::join(
            || number_of_fishes(7, days_remaining - fish_timer),
            || number_of_fishes(9, days_remaining - fish_timer));
        1 + a + b
    }
}

