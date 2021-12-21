use std::collections::HashSet;
use std::ops::Not;

const INPUT: &str = include_str!("../input");
const TEST: &str = include_str!("../test");

fn get_input(s: &str) -> (Filter, Image) {
    let filter = Filter {
        data: s
            .lines()
            .next()
            .unwrap()
            .chars()
            .map(Pixel::from_char)
            .collect(),
    };

    let mut image = Image::new(Pixel::Dark);

    for (y, row) in s.lines().skip(2).enumerate() {
        for (x, c) in row.chars().enumerate() {
            image.set(x as i32, y as i32, Pixel::from_char(c))
        }
    }

    (filter, image)
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Pixel {
    Dark,
    Light,
}

impl Pixel {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Pixel::Dark,
            '#' => Pixel::Light,
            _ => unreachable!(),
        }
    }
}

impl Not for Pixel {
    type Output = Self;
    fn not(self) -> Self {
        match self {
            Pixel::Dark => Pixel::Light,
            Pixel::Light => Pixel::Dark,
        }
    }
}

struct Filter {
    data: Vec<Pixel>,
}

impl Filter {
    fn apply(&self, area: [Pixel; 9]) -> Pixel {
        fn one(p: &Pixel) -> usize {
            match p {
                Pixel::Dark => 0,
                Pixel::Light => 1,
            }
        }

        assert_eq!(512, self.data.len());
        let mut index = 0;
        for neighbour in area.iter() {
            index <<= 1;
            index += one(neighbour);
        }

        self.data[index]
    }
}

#[derive(Default)]
struct Dimension {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl Dimension {
    fn fit(&mut self, x: i32, y: i32) {
        let delta = 4;
        self.x_min = i32::min(self.x_min, x - delta);
        self.x_max = i32::max(self.x_max, x + delta);
        self.y_min = i32::min(self.y_min, y - delta);
        self.y_max = i32::max(self.y_max, y + delta);
    }
}

struct Image {
    dim: Dimension,
    infinite: Pixel,
    pixels: HashSet<(i32, i32)>,
}

impl Image {
    fn new(infinite: Pixel) -> Self {
        Image {
            dim: Dimension::default(),
            infinite,
            pixels: HashSet::new(),
        }
    }

    fn set(&mut self, x: i32, y: i32, p: Pixel) {
        if p != self.infinite {
            self.pixels.insert((x, y));
            self.dim.fit(x, y);
        }
    }

    fn get(&self, x: i32, y: i32) -> Pixel {
        match self.pixels.contains(&(x, y)) {
            true => !self.infinite,
            false => self.infinite,
        }
    }

    fn get_area(&self, x: i32, y: i32) -> [Pixel; 9] {
        [
            self.get(x - 1, y - 1),
            self.get(x, y - 1),
            self.get(x + 1, y - 1),
            self.get(x - 1, y),
            self.get(x, y),
            self.get(x + 1, y),
            self.get(x - 1, y + 1),
            self.get(x, y + 1),
            self.get(x + 1, y + 1),
        ]
    }

    fn num_pixels_light(&self) -> usize {
        if self.infinite == Pixel::Light {
            panic!("infinite light pixels!");
        }
        self.pixels.len()
    }

    fn filter(&self, f: &Filter) -> Self {
        let new_infinite = f.apply([self.infinite; 9]);
        let mut res = Self::new(new_infinite);

        for y in self.dim.y_min..self.dim.y_max + 1 {
            for x in self.dim.x_min..self.dim.x_max + 1 {
                res.set(x, y, f.apply(self.get_area(x, y)))
            }
        }

        res
    }

    fn show(&self) {
        for y in self.dim.y_min..self.dim.y_max + 1 {
            for x in self.dim.x_min..self.dim.x_max + 1 {
                let c = match self.get(x, y) {
                    Pixel::Dark => '.',
                    Pixel::Light => '#',
                };
                print!("{}", c);
            }
            println!();
        }
        println!("________________________________________________________________________");
    }
}

fn filter(s: &str, num_filt: i32) -> usize {
    let (filter, mut image) = get_input(s);
    for _ in 0..num_filt {
        image = image.filter(&filter);
    }

    image.num_pixels_light()
}

fn main() {
    assert_eq!(35, filter(TEST, 2));
    println!("{}", filter(INPUT, 2));
    assert_eq!(3351, filter(TEST, 50));
    println!("{}", filter(INPUT, 50));
}
