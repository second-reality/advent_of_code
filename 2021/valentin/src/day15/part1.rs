use std::cmp::Reverse;
use std::fs;
use crate::helper::{neighbors_matrix2d_no_diag, parse_matrix_of_digits};

use priority_queue::PriorityQueue;


pub fn dijkstra(start:usize, end:usize, grid:&Vec<u32>, n_columns:usize, n_lines:usize) -> (Vec<u32>, Vec<usize>) {
    let mut dist:Vec<u32> = grid.iter().map(|x| u32::MAX).collect();
    dist[start] = 0;
    let mut prev:Vec<usize> = grid.iter().map(|x| usize::MAX).collect();
    let mut queue:PriorityQueue<usize, Reverse<u32>> = PriorityQueue::new();
    (0..grid.len()).for_each(|u| {
        queue.push(u, Reverse(dist[u]));
    });
    while !queue.is_empty() {
        let (node, _dist_to_node) = queue.pop().unwrap();
        if node == end { break; }
        let neighbors = neighbors_matrix2d_no_diag(node, n_columns, n_lines);
        for neighbor in neighbors {
            let alt = dist[node] + grid[neighbor];
            if alt < dist[neighbor] {
                dist[neighbor] = alt;
                prev[neighbor] = node;
                queue.change_priority(&neighbor, Reverse(alt));
            }
        }
    }
    (dist, prev)
}

pub fn solution() -> usize {
    let text = fs::read_to_string("src/day15/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim().split('\n').collect();
    let grid: Vec<u32> = parse_matrix_of_digits(lines);
    let start = 0;
    let end = grid.len()-1;

    let (distances, _) = dijkstra(start, end, &grid, 100, 100);
    distances[end] as usize
}