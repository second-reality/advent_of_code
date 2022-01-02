const NUM_OF_DAYS : i32 = 80;

fn main() {
    let input = include_str!("../input").trim();
    let initial_number_of_fishes = input.split(",").count() as i32;
    let total = input.split(",")
                     .map(|c| c.parse::<i32>().unwrap())
                     .map(|i| number_of_fishes(i, NUM_OF_DAYS))
                     .sum::<i32>();
    println!("{:#?}", total + initial_number_of_fishes);
}

fn number_of_fishes(fish_timer: i32, days_remaining: i32) -> i32 {
    if fish_timer >= days_remaining {
        0
    } else {
        1 + number_of_fishes(7, days_remaining - fish_timer) 
          + number_of_fishes(9, days_remaining - fish_timer)
    }
}

