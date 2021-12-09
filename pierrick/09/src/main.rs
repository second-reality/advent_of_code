struct HeightMap {
    width: i32,
    height: i32,
    data: Vec<u32>,
}

impl HeightMap {
    fn new() -> Self {
        HeightMap {
            width: 0,
            height: 0,
            data: Vec::new(),
        }
    }

    fn add_row(&mut self, row: &[u32]) {
        assert!(self.width == 0 || self.width as usize == row.len());
        if self.width == 0 {
            self.width = row.len() as i32;
        }
        self.height += 1;
        self.data.extend_from_slice(row);
    }

    fn value(&self, x: i32, y: i32) -> Option<u32> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return None;
        }

        let coord = y * self.width + x;
        Some(self.data[coord as usize])
    }

    fn is_lower_than_neighbours(&self, x: i32, y: i32) -> bool {
        fn lower_than(val: u32, neighbour: Option<u32>) -> bool {
            val < neighbour.unwrap_or(u32::max_value())
        }
        let val = self.value(x, y).unwrap();

        lower_than(val, self.value(x - 1, y))
            && lower_than(val, self.value(x + 1, y))
            && lower_than(val, self.value(x, y - 1))
            && lower_than(val, self.value(x, y + 1))
    }
}

fn get_input(s: &str) -> HeightMap {
    let mut res = HeightMap::new();
    for row in s.lines() {
        let data: Vec<u32> = row.chars().map(|c| c.to_digit(10).unwrap()).collect();
        res.add_row(&data);
    }
    res
}

fn part1(hm: &HeightMap) -> u32 {
    let mut res = 0;
    for x in 0..hm.width {
        for y in 0..hm.height {
            let lower = hm.is_lower_than_neighbours(x, y);
            if lower {
                res += hm.value(x, y).unwrap() + 1;
            }
        }
    }
    res
}

fn main() {
    let test = get_input(include_str!("../input_test"));
    let input = get_input(include_str!("../input"));
    println!("test {}", part1(&test));
    println!("{}", part1(&input));
}
