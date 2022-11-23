use std::collections::{HashMap, HashSet};
use std::fs;

pub fn parse_graph(lines: Vec<&str>) -> (Vec<Vec<usize>>, HashMap<&str, usize>) {
    let mut map_name_to_int: HashMap<&str, usize> = HashMap::new();
    let mut neighborhood: Vec<Vec<usize>> = Vec::new();
    let mut cur_index: usize = 0;
    for line in lines {
        let split: Vec<&str> = line.split('-').collect();
        if !map_name_to_int.contains_key(split[0]) {
            map_name_to_int.insert(split[0], cur_index);
            cur_index += 1;
            neighborhood.push(Vec::new());
        }
        if !map_name_to_int.contains_key(split[1]) {
            map_name_to_int.insert(split[1], cur_index);
            cur_index += 1;
            neighborhood.push(Vec::new());
        }
        let index0 = *map_name_to_int.get(split[0]).unwrap();
        let index1 = *map_name_to_int.get(split[1]).unwrap();
        neighborhood[index0].push(index1);
        neighborhood[index1].push(index0);
    }
    (neighborhood, map_name_to_int)
}

fn dfs(cur_node:usize, end:usize, being_visited:&mut Vec<bool>, once_nodes:&HashSet<usize>, neighbors:&Vec<Vec<usize>>, cur_path:&mut Vec<usize>, paths:&mut Vec<Vec<usize>>) {
    being_visited[cur_node] = once_nodes.contains(&cur_node);
    if cur_node == end {
        paths.push(cur_path.clone());
        being_visited[end] = false;
        return;
    }

    for neighbor in &neighbors[cur_node] {
        if !being_visited[*neighbor] {
            cur_path.push(*neighbor);
            dfs(*neighbor, end, being_visited, once_nodes, neighbors, cur_path, paths);
            cur_path.pop();
        }
    }
    being_visited[cur_node] = false;
}
fn find_all_path(start: usize, end: usize, neighborhood: Vec<Vec<usize>>, once_nodes:HashSet<usize>) -> Vec<Vec<usize>> {
    // init visited nodes for DFS and paths for result
    let mut paths: Vec<Vec<usize>> = Vec::new();
    let mut current_path:Vec<usize> = Vec::new();
    let mut being_visited:Vec<bool> = neighborhood.iter().map(|_| false).collect();
    current_path.push(start);

    dfs(start, end, &mut being_visited, &once_nodes, &neighborhood, &mut current_path, &mut paths);
    paths
}

pub fn solution() -> usize {
    let text = fs::read_to_string("src/day12/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim().split('\n').collect();
    let (graph, name_to_int) = parse_graph(lines);

    let start = name_to_int["start"];
    let end = name_to_int["end"];
    let small_caves: HashSet<usize> = name_to_int.into_iter()
        .filter(|(name, _)| (*name).chars().all(|c| c.is_lowercase()))
        .map(|(_, index)| index).collect();
    let paths = find_all_path(start, end, graph, small_caves);
    // paths.iter().for_each(|v| println!("{:?}", *v));
    paths.len()
}