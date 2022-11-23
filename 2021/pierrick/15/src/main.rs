use petgraph::algo::*;
use petgraph::graph::*;
use std::collections::HashMap;

type G = Graph<(), i32>;

struct Problem {
    graph: G,
    start: NodeIndex,
    end: NodeIndex,
}

fn get_input(s: &str, num_duplicates: i32) -> Problem {
    let mut res = Graph::new();
    let mut nodes = HashMap::new();

    let orig_size = s.lines().next().unwrap().len() as i32;
    let size = orig_size * (num_duplicates + 1);

    for (i, row) in s.lines().enumerate() {
        for (j, val) in row.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            let i = i as i32;
            let j = j as i32;
            let val = val as i32;
            let node = res.add_node(());
            nodes.insert((i, j), (node, val));
            for x_scale in 0..num_duplicates + 1 {
                for y_scale in 0..num_duplicates + 1 {
                    if x_scale == 0 && y_scale == 0 {
                        continue;
                    }
                    let dup_i = i + x_scale * orig_size;
                    let dup_j = j + y_scale * orig_size;
                    let mut dup_val = val + x_scale + y_scale;
                    if dup_val > 9 {
                        dup_val = dup_val % 10 + 1;
                    }
                    let node = res.add_node(());
                    nodes.insert((dup_i, dup_j), (node, dup_val));
                }
            }
        }
    }

    for i in 0..size {
        for j in 0..size {
            let current = nodes.get(&(i, j)).unwrap().0;
            [
                nodes.get(&(i - 1, j)), // left
                nodes.get(&(i + 1, j)), // right
                nodes.get(&(i, j - 1)), // up
                nodes.get(&(i, j + 1)), // down
            ]
            .into_iter()
            .flatten()
            .for_each(|(n, cost)| {
                res.add_edge(current, *n, *cost);
            });
        }
    }

    Problem {
        graph: res,
        start: nodes.get(&(0, 0)).unwrap().0,
        end: nodes.get(&(size - 1, size - 1)).unwrap().0,
    }
}

fn solution(s: &str, num_duplicates: i32) -> i32 {
    let p = get_input(s, num_duplicates);
    let shortest_path = astar(
        &p.graph,
        p.start,
        |finish| finish == p.end,
        |e| *e.weight(),
        |_| 0,
    )
    .unwrap();
    shortest_path.0
}

fn part1(s: &str) -> i32 {
    solution(s, 0)
}

fn part2(s: &str) -> i32 {
    solution(s, 4)
}

const TEST: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

fn main() {
    assert_eq!(40, part1(TEST));
    println!("{}", part1(include_str!("../input")));
    assert_eq!(315, part2(TEST));
    println!("{}", part2(include_str!("../input")));
}
