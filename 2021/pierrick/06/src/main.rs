fn get_input(s: &str) -> Vec<u8> {
    s.trim_end()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

const MAX_DAYS: usize = 9;

struct Population {
    count: [u64; MAX_DAYS],
}

impl Population {
    fn new(fishes: &[u8]) -> Self {
        let mut count = [0; MAX_DAYS];
        fishes.iter().for_each(|&x| count[x as usize] += 1);
        Population { count }
    }

    fn one_day(&mut self) {
        let mut passed = 0;
        for day in (0..MAX_DAYS).rev() {
            std::mem::swap(&mut self.count[day], &mut passed);
        }
        self.count[6] += passed;
        self.count[8] += passed;
    }

    fn size(&self) -> u64 {
        self.count.iter().sum()
    }
}

fn population_after_n_days(num_days: usize, fishes: &[u8]) -> u64 {
    let mut population = Population::new(fishes);
    (0..num_days).for_each(|_| population.one_day());
    population.size()
}

fn main() {
    let fishes_test = get_input(include_str!("../input_test.txt"));
    let fishes = get_input(include_str!("../input.txt"));
    println!("test: {}", population_after_n_days(80, &fishes_test));
    println!("{}", population_after_n_days(80, &fishes));
    println!("test: {}", population_after_n_days(256, &fishes_test));
    println!("{}", population_after_n_days(256, &fishes));
}
