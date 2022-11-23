use std::cmp::{min, max};

const MAX_VAL : usize = 1000;

fn main() {
    let input = include_str!("../input");
    let lines = parse_input(input);
    part1(&lines);
}

fn parse_input(input: &'static str) -> Vec<(usize, usize, usize, usize)> {
    let mut lines = Vec::new();
    for line in input.lines() {
         let v : Vec<usize> = line.split("->")
                            .map(|s| s.split(","))
                            .flatten()
                            .map(|s| s.trim())
                            .map(|s| s.parse::<usize>().unwrap())
                            .collect();

         assert_eq!(v.len(), 4);
         let coordinates : (usize, usize, usize, usize) = (v[0], v[1], v[2], v[3]);
         lines.push(coordinates);
    }
    return lines;
}


fn part1(input: &Vec<(usize, usize, usize, usize)>) {
    let mut diagram = vec![0; MAX_VAL * MAX_VAL];
    let mut diagram_part2 = vec![0; MAX_VAL * MAX_VAL];
    for (x0, y0, x1, y1) in input {
        update_diagram(&mut diagram, *x0, *y0, *x1, *y1, false);
        update_diagram(&mut diagram_part2, *x0, *y0, *x1, *y1, true);
    }

    let mut cmpt_part1 = 0;
    let mut cmpt_part2 = 0;
    for cell in &diagram {
        if *cell > 1 {
            cmpt_part1 += 1;
        }
    }
    for cell in &diagram_part2 {
        if *cell > 1 {
            cmpt_part2 += 1;
        }
    }
    println!("{}", cmpt_part1);
    println!("{}", cmpt_part2);
}

enum ORIENTATION { NW, NE, SW, SE }

fn update_diagram(diagram: &mut Vec<usize>, x0: usize, y0: usize, x1: usize, y1: usize, is_part_two: bool) {
    if x0 != x1 && y0 == y1  { // horizontal
        for coordinate in 0..(max(x0,x1) - min(x0, x1) + 1) {
            diagram[coordinate + min(x0, x1) + y0 * MAX_VAL] += 1;
        }
    }
    else if x0 == x1 && y0 != y1  { // vertical
       for coordinate in 0..(max(y0, y1) - min(y0, y1) + 1) {
            diagram[x0 + (coordinate + min(y0, y1)) * MAX_VAL] += 1;
        }
    }

    else if is_part_two {
        let diag_orientation : ORIENTATION;
        if x0 < x1 && y0 < y1 {
            diag_orientation = ORIENTATION::SW;
        } else if x0 > x1 && y0 > y1 {
            diag_orientation = ORIENTATION::NE;
        } else if x0 < x1 && y0 > y1 {
            diag_orientation = ORIENTATION::NW;
        } else if x0 > x1 && y0 < y1 {
            diag_orientation = ORIENTATION::SE;
        } else {
            panic!("shouldn't be here");
        }

        let offset = max(x0, x1) - min(x0, x1) + 1;
        match diag_orientation {
            ORIENTATION::SW => {
                for i in 0..offset {
                    diagram[x0 + i + (y0 + i) * MAX_VAL] += 1;
                }
            }
            ORIENTATION::SE => {
                for i in 0..offset {
                    diagram[x0 - i + (y0 + i) * MAX_VAL] += 1;
                }
            }
            ORIENTATION::NW => {
                for i in 0..offset {
                    diagram[x0 + i + (y0 - i) * MAX_VAL] += 1;
                }
            }
            ORIENTATION::NE => {
                for i in 0..offset {
                    diagram[x0 - i + (y0 - i) * MAX_VAL] += 1;
                }
            }
        }
    }

}

#[allow(dead_code)]
fn print_2d_array(array: &Vec<usize>, dim: usize) {
    for i in 0..dim {
        for j in 0..dim {
            print!("{}, ", array[j + i * MAX_VAL]);
        }
        println!("");
    }
}

