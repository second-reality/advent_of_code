fn main() {
    let input = include_str!("../input")
                    .trim()
                    .split(",")
                    .map(|c| c.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
    let max_val = input.iter().max().unwrap();
    let min_val = input.iter().min().unwrap();
    let mut min_cost_part1 = i32::MAX;
    let mut min_cost_part2 = i32::MAX;

    for offset in *min_val..*max_val {
        let cost_part1 = cost_part1(&input, offset);
        let cost_part2 = cost_part2(&input, offset);
        if cost_part1 < min_cost_part1 {
            min_cost_part1 = cost_part1;
        }
        if cost_part2 < min_cost_part2 {
            min_cost_part2 = cost_part2;
        }
    }

    println!("{}", min_cost_part1);
    println!("{}", min_cost_part2);

}

fn cost_part1(crab_positions: &Vec<i32>, offset: i32) -> i32 {
    let mut cost = 0;
    for pos in crab_positions {
        cost += (pos - offset).abs();
    }
    
    return cost;
}

fn cost_part2(crab_positions: &Vec<i32>, offset: i32) -> i32 {
    let mut cost = 0;
    for pos in crab_positions {
        let n = (pos - offset).abs();
        cost += n * (n + 1) / 2;
    }
    
    return cost;
}
