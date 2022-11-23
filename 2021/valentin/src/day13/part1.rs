use std::collections::HashSet;
use std::fs;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Point {
    pub(crate) x:i32,
    pub(crate) y:i32,
}

impl Point {
    fn from_str(line:&str) -> Point {
        let split:Vec<i32> = line.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
        Point {
            x:split[0],
            y:split[1]
        }
    }

    fn symmetric_along_x(&self, x:i32) -> Point {
        Point {
            x: x - (x - self.x).abs(),
            y:self.y,
        }
    }

    fn symmetric_along_y(&self, y:i32) -> Point {
        Point {
            x: self.x,
            y:y - (y - self.y).abs(),
        }
    }
}


pub fn parse_points_and_instructions(lines:Vec<&str>) -> (HashSet<Point>, Vec<&str>) {
    let mut points:HashSet<Point> = HashSet::new();
    let mut instructions_begin = false;
    let mut instructions:Vec<&str> = Vec::new();
    for line in lines {
        if line.is_empty() {
            instructions_begin = true;
            continue;
        }
        if instructions_begin { 
            instructions.push(line);
        } else {
            // point parsing
            points.insert(Point::from_str(line));
        }
    }
    (points, instructions)
}

pub fn apply_instruction(instruction:&str, points:HashSet<Point>) -> HashSet<Point>{
    if instruction.contains("x=") {
        let x = instruction.split("x=").last().unwrap().parse::<i32>().unwrap();
        points.into_iter().map(|p| p.symmetric_along_x(x)).collect()
    } else {
        let y = instruction.split("y=").last().unwrap().parse::<i32>().unwrap();
        points.into_iter().map(|p| p.symmetric_along_y(y)).collect()
    }
}

pub fn solution() -> usize {
    let text = fs::read_to_string("src/day13/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim().split('\n').collect();
    let (points, instructions) = parse_points_and_instructions(lines);
    let after_step1 = apply_instruction(instructions[0], points);
    after_step1.len()
}