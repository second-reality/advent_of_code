use std::fs;
use crate::day15::part1::dijkstra;
use crate::helper::parse_matrix_of_digits;

const N_COLUMNS:usize = 500;
const N_LINES:usize = 500;
const N:usize = N_COLUMNS * N_LINES;

const N_TILE_LINES:usize = 100;
const N_TILE_COLUMNS:usize = 100;


fn build_grid_from_tile(tile:Vec<u32>) -> Vec<u32>{
    let mut res:Vec<u32> = Vec::from([0; N]);
    for i in 0..N_LINES {
        for j in 0..N_COLUMNS {
            let index = i * N_COLUMNS + j;
            let index_tile = (i % N_TILE_LINES) * N_TILE_COLUMNS + (j % N_TILE_COLUMNS);
            let plus_i = (i / N_TILE_LINES) as u32;
            let plus_j = (j / N_TILE_COLUMNS) as u32;
            let new_val = tile[index_tile] + plus_i + plus_j;
            res[index] = if new_val > 9 { new_val - 9 } else { new_val };
        }
    }
    res
}
pub fn solution() -> usize {
    let text = fs::read_to_string("src/day15/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim().split('\n').collect();
    let tile: Vec<u32> = parse_matrix_of_digits(lines);
    let grid = build_grid_from_tile(tile);
    let start = 0;
    let end = grid.len()-1;

    let (distances, _) = dijkstra(start, end, &grid, N_COLUMNS, N_LINES);
    distances[end] as usize
}