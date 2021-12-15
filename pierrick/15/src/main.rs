use petgraph::algo::*;
use petgraph::graph::*;
use std::collections::HashMap;

type G = Graph<(), i32>;

struct Problem {
    graph: G,
    start: NodeIndex,
    end: NodeIndex,
}

fn get_input(s: &str) -> Problem {
    let mut res = Graph::new();
    let mut nodes = HashMap::new();

    let size = s.lines().next().unwrap().len() as i32;

    for (i, row) in s.lines().enumerate() {
        for (j, val) in row.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            let i = i as i32;
            let j = j as i32;
            let val = val as i32;
            let node = res.add_node(());
            nodes.insert((i, j), (node, val));
        }
    }

    for i in 0..size {
        for j in 0..size {
            let current = nodes.get(&(i, j)).unwrap().0;
            // left
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

fn part1(p: &Problem) -> i32 {
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
    let p = get_input(TEST);
    assert_eq!(40, part1(&p));
    let input = get_input(include_str!("../input"));
    println!("{}", part1(&input));
}
