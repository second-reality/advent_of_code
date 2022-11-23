use std::fs;

fn filter_with_bit_criteria(lines: &Vec<&str>, oxygen:bool) -> String {
    let mut candidates = lines.clone();
    let mut i: usize = 0;
    print!("to filter str   ");
    while candidates.len() != 1 {
        let n_bits_total = candidates.len();
        let n_bits_sets = candidates.iter().filter(|s| (*s).chars().nth(i).unwrap() == '1').count();
        let char_needed = if oxygen {
            if n_bits_sets >= n_bits_total / 2 { '1' } else { '0'}
        } else {
            if n_bits_sets >= n_bits_total / 2 { '0' } else { '1'}
        };
        print!("{}", char_needed);
        candidates = candidates.into_iter()
            .filter(|s|
                s.chars().nth(i).unwrap() == char_needed)
            .collect();
        i += 1;
    }
    println!();
    return candidates[0].parse().unwrap();
}

pub fn solution() -> u32 {
    let text = fs::read_to_string("src/day3/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim().split('\n').collect();

    let oxygen_str = filter_with_bit_criteria(&lines, true);
    println!("filtered string {}", oxygen_str);
    let co2_str = filter_with_bit_criteria(&lines, false);


    println!("filtered string {}", co2_str);

    let oxygen = u32::from_str_radix(&*oxygen_str, 2).unwrap();
    let co2 = u32::from_str_radix(&*co2_str, 2).unwrap();

    println!("filtered int {}", oxygen);
    println!("filtered int {}", co2);

    return oxygen * co2;
}