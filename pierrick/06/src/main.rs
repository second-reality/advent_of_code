fn get_input(s: &str) -> Vec<u8> {
    s.trim_end()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn after_one_day(population: &mut Vec<u8>) {
    let num_dead = population.iter().filter(|&p| *p == 0).count();
    let handle_one = |x| {
        if x == 0 {
            return 6;
        }
        x - 1
    };
    for i in 0..population.len() {
        population[i] = handle_one(population[i]);
    }
    population.append(&mut vec![8; num_dead]);
}

fn population_after_n_days(num_days: usize, population: &Vec<u8>) -> usize {
    let mut population = population.clone();
    for i in 0..num_days {
        println!("on day: {}", i);
        after_one_day(&mut population);
    }
    population.len()
}

fn main() {
    let population_test = get_input(include_str!("../input_test.txt"));
    let population = get_input(include_str!("../input.txt"));
    println!("test: {}", population_after_n_days(80, &population_test));
    println!("{}", population_after_n_days(80, &population));
    println!("test: {}", population_after_n_days(256, &population_test));
    println!("{}", population_after_n_days(256, &population));
}
