use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs;
use crate::day22::part1::parse_line;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Cuboid {
    coords: [i32; 6],
}

#[derive(Debug, Clone, Copy, EnumIter)]
enum Axis {
    X,
    Y,
    Z,
}

fn index_from_axis(axis: Axis) -> usize {
    match axis {
        Axis::X => 0,
        Axis::Y => 2,
        Axis::Z => 4,
    }
}

impl Cuboid {
    fn intersection_axis(&self, other: Cuboid, axis: Axis) -> Option<(i32, i32)> {
        let i = index_from_axis(axis);

        let inter_min = max(self.coords[i], other.coords[i]);
        let inter_max = min(self.coords[i + 1], other.coords[i + 1]);

        if inter_min <= inter_max {
            Option::Some((inter_min, inter_max))
        } else {
            Option::None
        }
    }


    fn get_intersection_with(&self, other: Cuboid) -> Option<Cuboid> {
        let mut coords = self.coords.clone();
        for axis in Axis::iter() {
            let index = index_from_axis(axis);
            let intersection = self.intersection_axis(other, axis);
            if intersection.is_none() {
                return Option::None;
            }
            let (ax_min, ax_max) = intersection.unwrap();
            coords[index] = ax_min;
            coords[index + 1] = ax_max;
        }
        Option::Some(Cuboid {
            coords
        })
    }

    fn without(&self, part:Cuboid) -> Vec<Cuboid>{
        let mut res:Vec<Cuboid> = Vec::new();
        let mut base = self.coords.clone();
        for axis in Axis::iter() {
            let i = index_from_axis(axis);
            if part.coords[i] > self.coords[i] {
                let mut coords_left = base.clone();
                coords_left[i + 1] = part.coords[i] - 1;
                res.push(Cuboid {coords:coords_left});
                base[i] = part.coords[i];
            }

            if part.coords[i + 1] < self.coords[i + 1]  {
                let mut coords_right = base.clone();
                coords_right[i] = part.coords[i + 1] + 1;
                res.push(Cuboid {coords:coords_right});
                base[i+1] = part.coords[i+1];
            }
        }
        res
    }

    fn count_points(&self) -> usize {
        let mut count:usize = 1;
        for i in (0..6).step_by(2) {
            count *= (self.coords[i + 1] - self.coords[i] + 1) as usize;
        }
        count
    }
}

fn insert_cuboid(cuboid:Cuboid, cuboids:&Vec<Cuboid>) -> Vec<Cuboid> {
    let mut new_cuboids = remove_cuboid(cuboid, cuboids);
    new_cuboids.push(cuboid);
    new_cuboids
}

fn  remove_cuboid(cuboid:Cuboid, cuboids:&Vec<Cuboid>) -> Vec<Cuboid> {
    cuboids.iter()
        .flat_map(|c| {
            let inter = c.get_intersection_with(cuboid);
            if inter.is_some() {
                c.without(inter.unwrap())
            } else {
                vec![*c]
            }
        }).collect()
}

pub fn solution() -> usize {
    let text = fs::read_to_string("src/day22/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim().split('\n').collect();
    let mut cuboids: Vec<Cuboid> = Vec::new();

    for (index, line) in lines.into_iter().enumerate() {
        let (on, x_min, x_max, y_min, y_max, z_min, z_max) = parse_line(line);
        let cuboid = Cuboid {
            coords: [
                x_min,
                x_max,
                y_min,
                y_max,
                z_min,
                z_max
            ]
        };

        cuboids = if on {
             insert_cuboid(cuboid,& cuboids)
        } else {
            remove_cuboid(cuboid, & cuboids)
        };
    }

    cuboids.iter().map(|c| c.count_points()).sum()
}