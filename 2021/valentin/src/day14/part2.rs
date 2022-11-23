use std::collections::HashMap;
use std::fs;
use crate::day14::part1::{Pair, parse_transformations};

impl Pair {
    fn to_string(&self) -> String {
        return format!("{}{}", self.left, self.right);
    }
}

fn count_pairs_next_step(count_pairs_init:HashMap<Pair, usize>, transformations:&HashMap<Pair, char>, occurrences:&mut HashMap<char, usize>) -> HashMap<Pair, usize> {
    let mut res = count_pairs_init.clone();
    transformations.iter()
        .for_each(|(pair, new_char)|{
            let count = *count_pairs_init.get(pair).unwrap();
            if count == 0 { return ;}
            occurrences.entry(*new_char).and_modify(|c| *c += count);
            let new_pair1 = Pair {
                left: pair.left,
                right: *new_char,
            };
            res.entry(new_pair1).and_modify(|c| *c += count);
            let new_pair2 = Pair {
                left: *new_char,
                right: pair.right,
            };
            res.entry(new_pair2).and_modify(|c| *c += count);
            res.entry(*pair).and_modify(|c| *c -= count);
        });
    res
}

fn parse_occurrences_and_transformations(lines:Vec<&str>) -> (HashMap<char, usize>, HashMap<Pair, char>){
    let mut occurrences:HashMap<char, usize> = HashMap::new();
    let mut res: HashMap<Pair, char> = HashMap::new();
    for i in 2..lines.len() {
        let split: Vec<&str> = lines[i].split(" -> ").collect();
        let pair: Vec<char> = split[0].chars().collect();
        occurrences.insert(pair[0], 0);
        let pair = Pair {
            left: pair[0],
            right: pair[1],
        };
        res.insert(pair, split[1].chars().last().unwrap());
    }
    (occurrences, res)
}

fn count_pair_in_str(pair:Pair, string:&str) -> usize {
    let mut count:usize = 0;
    let chars:Vec<char> = string.chars().collect();
    for i in 0..(chars.len()-1) {
        if pair.left == chars[i] && pair.right == chars[i+1] {
            count +=1;
        }
    }
    count
}

pub fn solution() -> usize {
    let text = fs::read_to_string("src/day14/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim()
        .split('\n')
        .collect();
    let template= lines[0];
    let (mut occurrences, transformations) = parse_occurrences_and_transformations(lines);
    template.chars().for_each(|c| {
        *occurrences.entry(c).or_insert(1) += 1;
    });
    println!("init occurrences {:?}", occurrences);

    let mut count_pairs: HashMap<Pair, usize> = transformations.iter()
        .map(|(k, v)| {
            (*k, count_pair_in_str(*k, template))
        })
        .collect();

    for _ in 0..40 {
        count_pairs = count_pairs_next_step(count_pairs, &transformations, &mut occurrences);
    }
    println!("occurrences {:?}", occurrences);
    let most_common = occurrences.iter().map(|(_, count)| *count).max().unwrap();
    let less_common = occurrences.iter().map(|(_, count)| *count).min().unwrap();
    most_common - less_common
}