use std::collections::HashSet;
use std::fs;

const MAT_N: usize = 5;
const N: usize = 100;

pub fn parse_matrices(lines: &Vec<String>) -> Vec<Vec<Vec<i32>>> {
    let mut res: Vec<Vec<Vec<i32>>> = Vec::new();
    for i in (2..lines.len()).step_by(MAT_N + 1) {
        let grid = lines[i..(i + MAT_N)].iter()
            .map(|line|
                line.split(' ')
                    .map(|num| num.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>())
            .collect::<Vec<Vec<i32>>>();
        res.push(grid);
    }
    return res;
}

pub fn bingo(grid: &Vec<Vec<i32>>, num_to_check: &HashSet<i32>) -> i32 {
    let mut marked = [[false; MAT_N]; MAT_N];
    let mut win_score: i32 = 0;
    for i in 0..MAT_N {
        for j in 0..MAT_N {
            marked[i][j] = num_to_check.contains(&grid[i][j]);
            if !marked[i][j] { win_score += grid[i][j] }
        }
    }

    // checks for rows
    for i in 0..MAT_N {
        if marked[i].iter().all( |x| (*x) == true) {
            // println!("Num crossed {:?}", num_to_check);
            // println!("Grid {:?} wins in with line {}", grid, i);
            return win_score;
        }
    }

    // checks for columns
    for j in 0..MAT_N {
        let mut bingo = true;
        for i in 0..MAT_N {
            if !marked[i][j] {
                bingo = false;
                break;
            }
        }
        if bingo {
            // println!("Grid {:?} wins in with column {}", grid, j);
            return win_score;
        }
    }
    return -1;
}

pub fn solution() -> i32 {
    let text = fs::read_to_string("src/day4/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<String> = text.trim().split('\n').map(|line| line.trim().replace("  ", " ")).collect();
    println!("{}", lines[38]);

    let numbers_to_cross: Vec<i32> = lines[0].split(',').map(|s| s.parse::<i32>().unwrap()).collect();
    let grids = parse_matrices(&lines);
    let mut set: HashSet<i32> = HashSet::new();
    for i in 0..N {
        set.insert(numbers_to_cross[i]);
        let winners: Vec<i32> = grids.iter()
            .map(|grid| bingo(&grid, &set))
            .filter(|x| (*x) != -1)
            .collect();
        if winners.len() == 1 {
            return winners[0] * numbers_to_cross[i];
        }
    }
    println!("Probably error, no one wins");
    return -1;
}