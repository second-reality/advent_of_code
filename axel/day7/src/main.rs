fn main() {
    let input = include_str!("../input")
                    .trim()
                    .split(",")
                    .map(|c| c.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
    let max_val = input.iter().max().unwrap();
    let min_val = input.iter().min().unwrap();
    let mut min_cost = i32::MAX;

    for offset in *min_val..*max_val {
        let ccost = cost(&input, offset);
        if ccost < min_cost {
            min_cost = ccost;
        }
    }

    println!("{}", min_cost);

}

fn cost(crab_positions: &Vec<i32>, offset: i32) -> i32 {
    let mut cost = 0;
    for pos in crab_positions {
        cost += (pos - offset).abs();
    }
    
    return cost;
}
