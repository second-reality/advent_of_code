use std::collections::HashSet;

struct Grid {
    trees: Vec<u8>,
    n_rows: usize,
    n_cols: usize,
}

impl Grid {
    fn new(input: String) -> Self {
        let mut n_cols = 0;
        let trees: Vec<u8> = input
            .trim()
            .split('\n')
            .flat_map(|line| {
                n_cols = line.len();
                line.chars().map(|h| h.to_digit(10).unwrap() as u8)
            })
            .collect();
        let n_rows = trees.len() / n_cols;
        Grid {
            trees,
            n_rows,
            n_cols,
        }
    }

    fn tree_visibles(&self) -> HashSet<usize> {
        let mut visible_trees = HashSet::new();
        let mut max_h;
        for i in 0..self.n_rows {
            // catch the corresponding line
            let row_start = i * self.n_cols;
            let row = self.trees[row_start..(row_start + self.n_cols)]
                .iter()
                .enumerate();

            // iterate it left to right and collect visible trees
            visible_trees.insert(row_start);
            max_h = self.trees[row_start];
            row.clone().skip(1).for_each(|(j, &high)| {
                if high > max_h {
                    visible_trees.insert(row_start + j);
                    max_h = high;
                }
            });

            // iterate right to left and collect visible trees
            visible_trees.insert(row_start + self.n_cols - 1);
            max_h = self.trees[row_start + self.n_cols - 1];
            row.rev().skip(1).for_each(|(j, &high)| {
                if high > max_h {
                    visible_trees.insert(row_start + j);
                    max_h = high;
                }
            });
        }
        for j in 0..self.n_cols {
            let column = self.trees.iter().enumerate().skip(j).step_by(self.n_cols);
            // iterate up to down
            visible_trees.insert(j);
            max_h = self.trees[j];
            column.clone().skip(1).for_each(|(index, &high)| {
                if high > max_h {
                    visible_trees.insert(index);
                    max_h = high;
                }
            });
            // iterate down to up
            let col_end = (self.n_rows - 1) * self.n_cols + j;
            visible_trees.insert(col_end);
            max_h = self.trees[col_end];
            column.rev().skip(1).for_each(|(index, &high)| {
                if high > max_h {
                    visible_trees.insert(index);
                    max_h = high;
                }
            });
        }
        visible_trees
    }

    fn tree_score(&self, index: usize) -> usize {
        let high = self.trees[index];
        let line_min = index / self.n_cols * self.n_cols;
        let line_max = line_min + self.n_cols - 1;
        let mut dr = 0;
        let right = self.trees[index..=line_max]
            .iter()
            .skip(1)
            .map_while(|&h| {
                if h < high {
                    Some(1)
                } else {
                    dr = 1;
                    None
                }
            })
            .sum::<usize>();
        let mut dl = 0;
        let left = self.trees[line_min..index]
            .iter()
            .rev()
            .map_while(|&h| {
                if h < high {
                    Some(1)
                } else {
                    dl = 1;
                    None
                }
            })
            .sum::<usize>();
        let mut dd = 0;
        let down = self
            .trees
            .iter()
            .skip(index)
            .step_by(self.n_cols)
            .skip(1)
            .map_while(|&h| {
                if h < high {
                    Some(1)
                } else {
                    dd = 1;
                    None
                }
            })
            .sum::<usize>();
        let mut du = 0;
        let up = self
            .trees
            .iter()
            .rev()
            .skip(self.trees.len() - index - 1)
            .step_by(self.n_cols)
            .skip(1)
            .map_while(|&h| {
                if h < high {
                    Some(1)
                } else {
                    du = 1;
                    None
                }
            })
            .sum::<usize>();
        (left + dl) * (right + dr) * (down + dd) * (up + du)
    }
}

pub fn part1(input: String) -> usize {
    let grid = Grid::new(input);
    grid.tree_visibles().len()
}

pub fn part2(input: String) -> usize {
    let grid = Grid::new(input);
    let visible_dists = grid.tree_visibles();
    visible_dists
        .into_iter()
        .map(|index| grid.tree_score(index))
        .max()
        .unwrap()
}

pub const EXPECTED1: usize = 21;
pub const EXPECTED2: usize = 8;
