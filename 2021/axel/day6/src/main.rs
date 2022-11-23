const NUM_OF_DAYS : usize = 257;

fn main() {
    let input = include_str!("../input").trim();

    let mut tab = vec![-1; NUM_OF_DAYS * 9];

    let initial_number_of_fishes = input.split(",").count() as i64;
    let total = input.split(",")
                     .map(|c| c.parse::<i64>().unwrap())
                     .map(|i| number_of_fishes(&mut tab, i, NUM_OF_DAYS-1))
                     .sum::<i64>();
    println!("total : {}", total);
    println!("{:#?}", total + initial_number_of_fishes);
    //print_2d_array(&tab, 9, NUM_OF_DAYS);
}

fn number_of_fishes(tab: &mut Vec<i64>, fish_timer: i64, days_remaining: usize) -> i64 {
    if tab[fish_timer as usize + days_remaining * 9] != -1 {
        return tab[fish_timer as usize + days_remaining * 9];
    }
    
    if fish_timer >= days_remaining as i64 {
        tab[fish_timer as usize + days_remaining * 9] = 0;
        return 0;
    } else {
        let a = number_of_fishes(tab, 7, days_remaining - (fish_timer as usize));
        let b = number_of_fishes(tab, 9, days_remaining - (fish_timer as usize));
        tab[fish_timer as usize + days_remaining * 9] = 1 + a + b;
        return tab[fish_timer as usize + days_remaining * 9];
    }
}

