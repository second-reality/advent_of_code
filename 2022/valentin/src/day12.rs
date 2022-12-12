use std::collections::{HashMap, VecDeque};

type Data = (usize, usize, u8);
struct Grid {
    squares: Vec<Vec<u8>>,
    start: Data,
    end: Data,
}
impl Grid {
    fn new(input: String) -> Self {
        let mut start = (0, 0, 0);
        let mut end = (0, 0, 0);
        let squares: Vec<Vec<u8>> = input
            .trim()
            .split('\n')
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
                        'S' => {
                            start = (i, j, 'a' as u8);
                            'a'
                        }
                        'E' => {
                            end = (i, j, 'z' as u8);
                            'z'
                        }
                        other => other,
                    } as u8)
                    .collect()
            })
            .collect();
        Grid {
            squares,
            start,
            end,
        }
    }
    fn neighbors_of(&self, i: usize, j: usize) -> Vec<Data> {
        let mut res: Vec<Data> = Vec::new();
        if i != 0 {
            res.push((i - 1, j, self.squares[i - 1][j]));
        }
        if i != self.squares.len() - 1 {
            res.push((i + 1, j, self.squares[i + 1][j]));
        }
        if j != 0 {
            res.push((i, j - 1, self.squares[i][j - 1]));
        }
        if j != self.squares[0].len() - 1 {
            res.push((i, j + 1, self.squares[i][j + 1]))
        }
        res
    }
}
fn dijkstra(grid: Grid, is_part_one: bool) -> usize {
    let mut queue: VecDeque<Data> = VecDeque::new();
    if is_part_one {
        queue.push_back(grid.start);
    } else {
        queue.extend(grid.squares.iter().enumerate().flat_map(|(i, line)| {
            line.iter().enumerate().filter_map(move |(j, high)| {
                if *high == 'a' as u8 {
                    Some((i, j, *high))
                } else {
                    None
                }
            })
        }));
    }
    let mut prev: HashMap<Data, Data> = HashMap::new();
    queue.iter().for_each(|c| {
        prev.insert(*c, *c);
    });
    while !queue.is_empty() {
        let cur = queue.pop_front().unwrap();
        if cur == grid.end {
            break;
        }
        for nei in grid.neighbors_of(cur.0, cur.1) {
            if nei.2 <= cur.2 + 1 && !prev.contains_key(&nei) {
                prev.insert(nei, cur);
                queue.push_back(nei);
            }
        }
    }
    let mut res = 0;
    let mut cur = grid.end;
    while (is_part_one && cur != grid.start) || (cur.2 != 'a' as u8) {
        res += 1;
        cur = *prev.get(&cur).unwrap();
    }
    res
}
pub fn part1(input: String) -> usize {
    let grid = Grid::new(input);
    dijkstra(grid, true)
}

pub fn part2(input: String) -> usize {
    let grid = Grid::new(input);
    dijkstra(grid, false)
}

pub const EXPECTED1: usize = 31;
pub const EXPECTED2: usize = 29;
