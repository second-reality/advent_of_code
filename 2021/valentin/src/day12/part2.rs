use std::collections::HashSet;
use std::fs;
use crate::day12::part1::parse_graph;

fn dfs2(cur_node:usize, end:usize, being_visited:&mut Vec<bool>, once_nodes:&HashSet<usize>, neighbors:&Vec<Vec<usize>>, cur_path:&mut Vec<usize>, paths:&mut Vec<Vec<usize>>, allowed_twice:usize, mut count:u8) {
    being_visited[cur_node] = if cur_node == allowed_twice {
        count += 1;
        count >= 2
    } else {
        once_nodes.contains(&cur_node)
    };

    if cur_node == end {
        paths.push(cur_path.clone());
        being_visited[end] = false;
        return;
    }

    for neighbor in &neighbors[cur_node] {
        if !being_visited[*neighbor] {
            cur_path.push(*neighbor);
            dfs2(*neighbor, end, being_visited, once_nodes, neighbors, cur_path, paths, allowed_twice, count);
            cur_path.pop();
        }
    }
    being_visited[cur_node] = false;
}
fn find_all_path2(start: usize, end: usize, neighborhood: Vec<Vec<usize>>, once_nodes:HashSet<usize>) -> Vec<Vec<usize>> {
    // init visited nodes for DFS and paths for result
    let mut paths: Vec<Vec<usize>> = Vec::new();
    let mut current_path:Vec<usize> = Vec::new();
    let mut being_visited:Vec<bool> = neighborhood.iter().map(|_| false).collect();
    current_path.push(start);

    once_nodes.iter()
        .filter(|node| **node != end && **node != start)
        .for_each(|cur_node|
                      dfs2(start, end, &mut being_visited, &once_nodes, &neighborhood, &mut current_path, &mut paths, *cur_node, 0)
        );

    paths
}

pub fn solution() -> usize {
    let text = fs::read_to_string("src/day12/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim().split('\n').collect();
    let (graph, name_to_int) = parse_graph(lines);
    println!("name to int {:?}", name_to_int);
    let start = name_to_int["start"];
    let end = name_to_int["end"];
    let small_caves: HashSet<usize> = name_to_int.into_iter()
        .filter(|(name, _)| (*name).chars().all(|c| c.is_lowercase()))
        .map(|(_, index)| index).collect();
    let mut paths = find_all_path2(start, end, graph, small_caves);
    // remove duplicate using sort + dedup
    paths.sort();
    paths.dedup();
    paths.len()
}