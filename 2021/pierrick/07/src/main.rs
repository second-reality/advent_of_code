fn get_input(s: &str) -> Vec<i64> {
    s.trim_end()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn fuel_for_position(positions: &[i64], target: i64, cost: fn(i64) -> i64) -> i64 {
    positions.iter().map(|p| cost((p - target).abs())).sum()
}

fn cost_part1(distance: i64) -> i64 {
    distance
}

fn cost_part2(distance: i64) -> i64 {
    // sum 1,n = n*(n+1)/2
    distance * (distance + 1) / 2
}

fn min_fuel(positions: &[i64], cost: fn(i64) -> i64) -> i64 {
    let min_position: i64 = *positions.iter().min().unwrap();
    let max_position: i64 = *positions.iter().max().unwrap();

    (min_position..max_position + 1)
        .map(|p| fuel_for_position(positions, p, cost))
        .min()
        .unwrap()
}

fn main() {
    let input = get_input(include_str!("../input.txt"));
    let test = get_input("16,1,2,0,4,2,7,1,2,14");
    println!("test {}", min_fuel(&test, cost_part1));
    println!("{}", min_fuel(&input, cost_part1));
    println!("test {}", min_fuel(&test, cost_part2));
    println!("{}", min_fuel(&input, cost_part2));
}
