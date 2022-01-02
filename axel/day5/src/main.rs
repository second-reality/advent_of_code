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
    let mut diagram : [usize; MAX_VAL * MAX_VAL] = [0; MAX_VAL * MAX_VAL];
    for (x0, y0, x1, y1) in input {
        update_diagram(&mut diagram, *x0, *y0, *x1, *y1);
    }

    let mut cmpt = 0;
    for cell in &diagram {
        if *cell > 1 {
            cmpt += 1;
        }
    }
    println!("{}", cmpt);
}

fn update_diagram(diagram: &mut [usize; MAX_VAL * MAX_VAL], x0: usize, y0: usize, x1: usize, y1: usize) {
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
}

#[allow(dead_code)]
fn print_2d_array(array: &[usize; MAX_VAL * MAX_VAL]) {
    for i in 0..MAX_VAL {
        for j in 0..MAX_VAL {
            print!("{}, ", array[j + i * MAX_VAL]);
        }
        println!("");
    }
}

