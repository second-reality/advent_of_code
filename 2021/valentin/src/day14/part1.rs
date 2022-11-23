use std::collections::HashMap;
use std::fs;
use std::fmt;
use std::fmt::{Formatter, write};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Pair {
    pub left: char,
    pub right: char,
}

impl fmt::Debug for Pair {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.left, self.right)
    }
}

pub fn parse_transformations(lines: Vec<&str>) -> HashMap<Pair, char> {
    let mut res: HashMap<Pair, char> = HashMap::new();
    for i in 2..lines.len() {
        let split: Vec<&str> = lines[i].split(" -> ").collect();
        let pair: Vec<char> = split[0].chars().collect();
        let pair = Pair {
            left: pair[0],
            right: pair[1],
        };
        res.insert(pair, split[1].chars().last().unwrap());
    }
    res
}

pub fn solution() -> i32 {
    let text = fs::read_to_string("src/day14/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim().split('\n').collect();
    let mut template: Vec<char> = lines[0].chars().collect();
    let transformations = parse_transformations(lines);

    for _ in 0..10 {
        let last_char = template.last().unwrap().clone();
        template = template.iter()
            .zip(template.iter().skip(1))
            .flat_map(|(left, right)| {
                let pair = Pair { left: *left, right: *right };
                vec![*left, *transformations.get(&pair).unwrap()]
            })
            .collect();
        template.push(last_char);
    }

    let occurrences = template.iter().fold(HashMap::new(), |mut acc, elem| {
        *acc.entry(elem).or_insert(0) += 1;
        acc
    });
    println!("occurrences {:?}", occurrences);
    let most_common = occurrences.iter().map(|(_, count)| *count).max().unwrap();
    let less_common = occurrences.iter().map(|(_, count)| *count).min().unwrap();
    most_common - less_common
}