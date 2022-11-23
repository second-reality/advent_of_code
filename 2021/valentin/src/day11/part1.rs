use std::collections::HashSet;
use std::fs;


const N_COLUMNS: i32 = 10;
const N_LINES: i32 = 10;
const N:usize = 100;

fn get_neighbors(index: i32) -> Vec<usize> {
    let (i, j) = (index / N_COLUMNS, index % N_COLUMNS);
    let mut res: Vec<usize> = Vec::new();
    for di in -1..2 {
        for dj in -1..2 {
            if di == 0 && dj == 0 { continue; }
            if (0..N_LINES).contains(&(i + di)) && (0..N_COLUMNS).contains(&(j + dj)) {
                res.push(((i + di) * N_COLUMNS + j + dj) as usize)
            }
        }
    }
    res
}

fn dfs_charged(stack: &mut Vec<usize>, grid: &mut Vec<u32>){
    let mut already_flashed_octopuses: HashSet<usize> = stack.iter().map(|x| *x).collect();
    while !stack.is_empty() {
        // it flashes
        let cur_index = stack.pop().unwrap();
        let adjacent_indices = get_neighbors(cur_index as i32);
        for adjacent_index in adjacent_indices {
            grid[adjacent_index] += 1;
            if grid[adjacent_index] > 9 && !already_flashed_octopuses.contains(&adjacent_index) {
                already_flashed_octopuses.insert(adjacent_index);
                stack.push(adjacent_index);
            }
        }
    }
}

pub fn step_and_count_flash(grid: &mut Vec<u32>) -> usize {
    let mut charged_octopus: Vec<usize> = Vec::new();
    for i in 0..N {
        grid[i] += 1;
        if grid[i] > 9 {
            charged_octopus.push(i);
        }
    }
    // start DFS for each charged octopus
    dfs_charged(&mut charged_octopus, grid);

    let mut res:usize =0;
    for i in 0..N {
        if grid[i] > 9 {
            grid[i] = 0;
            res += 1;
        }
    }
    res
}

fn print_grid(grid:&Vec<u32>) {
    for i in 0..N {
        if i % 10 == 0 { println!();}
        print!("{} ", grid[i]);
    }
    println!();
}

pub fn solution() -> usize {
    let text = fs::read_to_string("src/day11/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim().split('\n').collect();

    let mut numbers: Vec<u32> = lines.into_iter()
        .flat_map(|s|
            s.chars().map(|c| c.to_digit(10).unwrap()))
        .collect();
    let mut count_flashes: usize = 0;
    // println!("before any step");
    print_grid(&numbers);
    for i in 0..100 {
        // println!("---------------------------------------");
        count_flashes += step_and_count_flash(&mut numbers);
        // println!("grid after step {}:", i + 1);
        // print_grid(&numbers);
    }
    count_flashes
}