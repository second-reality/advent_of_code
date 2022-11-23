use std::collections::HashSet;
use std::fs;
use crate::helper::{parse_matrix_of_digits, neighbors_matrix2d_no_diag};

const N_COLUMNS:i32 = 100;
const N_LINES:i32 = 100;

fn get_low_points(numbers:&Vec<u32>) -> HashSet<usize> {
    let mut low_points:HashSet<usize> = HashSet::new();
    for i in 0..numbers.len() {
        if get_neighbors(i as i32).into_iter().all(|neighbor| numbers[neighbor] > numbers[i]) {
            low_points.insert(i);
        }
    };
    return low_points;
}

// parcours en profondeur
fn dfs_basin(points:&Vec<u32>, visited_points:&mut HashSet<usize>, starting_point:usize) -> i64 {
    // ce bassin a déjà été parcouru
    if visited_points.contains(&starting_point) {
        return -1;
    }
    let mut basin_size:i64 = 0;
    let mut stack:Vec<usize> = Vec::new();
    stack.push(starting_point);
    visited_points.insert(starting_point);
    while !stack.is_empty() {
        let point = stack.pop().unwrap();
        basin_size += 1;
        let neighbors = neighbors_matrix2d_no_diag(point, N_COLUMNS, N_LINES);
        for neighbor in neighbors {
            if points[neighbor] != 9 && !visited_points.contains(&neighbor) {
                stack.push(neighbor);
                visited_points.insert(neighbor);
            }
        }
    }
    return basin_size;
}

pub fn solution() -> i64 {
    let text = fs::read_to_string("src/day9/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim().split('\n').collect();

    let numbers:Vec<u32> = parse_matrix_of_digits(lines);

    let mut visited_points:HashSet<usize> = HashSet::new();
    let low_points = get_low_points(&numbers);

    let mut basin_sizes:Vec<i64> = Vec::new();

    for low_point in low_points {
        basin_sizes.push(dfs_basin(&numbers, &mut visited_points, low_point))
    }
    basin_sizes.sort();
    basin_sizes.reverse();
    println!("size of basins {:?}", basin_sizes);
    return basin_sizes[0] * basin_sizes[1] * basin_sizes[2];
}