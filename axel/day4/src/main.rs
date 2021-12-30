type Grid = Vec<Vec<(i32, bool)>>;
type Grids = Vec<Grid>;

fn main() {
    let input = include_str!("../input");
    let (drawn_numbers, mut grids) = parse_input(input);

    let mut i = 0;
    while !someone_win(&grids) {
        update_grids(&mut grids, drawn_numbers[i]);
        i += 1;
    }

    let last_drawn_number = drawn_numbers[i-1];
    println!("{}", get_score_of_winner(&grids, last_drawn_number));

    // Part 2
    while grids.iter().map(|grid| if wins(grid) { 0 } else { 1 })
                       .sum::<i32>() != 1 {
        update_grids(&mut grids, drawn_numbers[i]);
        i += 1;
    }

    let mut last_grid = get_last_winner(&grids);
    while !wins(&last_grid) {
        update_grid(&mut last_grid, drawn_numbers[i]);
        i += 1;
    }

    let last_drawn_number = drawn_numbers[i-1];
    println!("{}", score(&last_grid, last_drawn_number));
}

fn parse_input(input: &'static str) -> (Vec<i32>, Grids) {
    // first line
    let mut line_iter = input.lines();
    let drawn_numbers : Vec<i32> = line_iter.next().unwrap().split(",")
                                            .map(|n| n.parse::<i32>().unwrap())
                                            .collect();

    let mut grids : Grids = Vec::new();
    for line in line_iter {
        if line == "" {
            grids.push(Vec::new()); // new grid
        }
        else {
            let bingo_line = line.split_whitespace()
                                 .map(|n| (n.parse::<i32>().unwrap(), false))
                                 .collect();
            let len = grids.len();
            grids[len-1].push(bingo_line);
        }
    }

    return (drawn_numbers, grids);
}

fn someone_win(grids: &Grids) -> bool {
    for grid in grids {
        if wins(&grid) {
            return true;
        }
    }
    return false;
}

fn get_last_winner(grids: &Grids) -> Grid {
    for grid in grids {
        if !wins(&grid) {
            return grid.clone();
        }
    }
    panic!("shouldn't be here");
}

fn wins(grid: &Grid) -> bool {
    let mut victory = true;

    // check rows
    for row in grid {
        for cell in row {
            victory = victory && cell.1;
        }
        if victory {
            return true;
        }
        victory = true;
    }

    // check cols
    for j in 0..grid[0].len() {
        for i in 0..grid.len() {
            victory = grid[i][j].1 && victory;
        }
        if victory {
            return true;
        }
        victory = true;
    }

    return false;
}


fn update_grids(grids: &mut Grids, drawn_number: i32) {
    for grid in grids {
        update_grid(grid, drawn_number);
    }
}

fn update_grid(grid: &mut Grid, drawn_number: i32) {
    for row in grid {
        for cell in row {
            if cell.0 == drawn_number {
                cell.1 = true;
            }
        }
    }
}

fn get_score_of_winner(grids: &Grids, last_drawn_number: i32) -> i32 {
    for grid in grids {
        if wins(&grid) {
            return score(&grid, last_drawn_number);
        }
    }
    return -1;
}

fn score(grid: &Grid, last_drawn_number: i32) -> i32 {
    let mut score = 0;
    for cell in grid.into_iter().flatten() {
        if !cell.1 {
            score += cell.0;
        }
    }
    return score * last_drawn_number;
}
